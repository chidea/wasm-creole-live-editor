#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use log::debug;

use wasm_bindgen::JsCast;
use web_sys::{
  Event, InputEvent, HtmlTextAreaElement,
};
use js_sys::{
  Date,
};
use gloo_timers::future::TimeoutFuture;
use sycamore::{
  prelude::*,
  builder::prelude::*,
  futures::spawn_local_scoped,
};
use sycamore_router::{Router, HistoryIntegration, };

use creole_nom::prelude::*;

mod route;
use route::AppRoutes;

// todo : light/dark theme
#[derive(Debug, Default, Clone)]
pub struct AppState {
  pub theme: RcSignal<Theme>,
}

#[derive(Debug, Clone)]
pub enum Theme {
  Light,
  Dark,
}
impl Default for Theme {
  fn default() -> Self {
    Theme::Light
  }
}

fn child<G: Html>(
  Comp: &dyn Fn(Scope, ()) -> View<G>,
) -> impl Fn(Scope, ()) -> View<G> + '_ {
  move |cx, _| {
      view! { cx,
        Comp {}
      }
  }
}

fn measure(perf: &web_sys::Performance, name: &str, s: &str, e: &str){
  if let Ok(_) = perf.measure_with_start_mark_and_end_mark(name, s, e) {
    let m : web_sys::PerformanceMeasure = perf.get_entries_by_name_with_entry_type(name, "measure").get(0).unchecked_into();
    debug!("{} : {}ms", m.name(), m.duration());
    // perf.clear_marks();
    // perf.clear_measures();
  }
}

fn creole_filled<'a, G: Html>(cx: Scope<'a>, tag: &str, t: Vec<ICreole<'a>>) -> G {
  let h = G::element_from_tag(tag);
  for c in t {
    if let Some(n) = CreoleItem(cx, c).as_node() {
      h.append_child(n);
    }
  }
  h
}
fn creole_as_node<'a, G: Html>(cx: Scope<'a>, tag: &str, t: Vec<ICreole<'a>>) -> View<G> {
  View::new_node(creole_filled(cx, tag, t))
}

#[component]
fn CreoleItem<'a, G: Html>(cx: Scope<'a>, i: ICreole<'a>) -> View<G> {
  match i {
    ICreole::Heading(l, t) => creole_as_node(cx, &format!("h{l}"), t),
    ICreole::Bold(t) => view! { cx, strong { (format!("{t}")) } },
    ICreole::Italic(t) => view! { cx, i { (format!("{t}")) } },
    ICreole::Text(t) => view! { cx, span { (format!("{t}")) } },
    ICreole::DontFormat(t) => view! { cx, pre { (format!("{t}"))  } },
    ICreole::Link(href, t) => view! { cx, a(href=format!("{href}")) { (format!("{t}")) } },
    ICreole::Line(l) => creole_as_node(cx, "p", l),
    ICreole::Image(src, t) => if t.is_empty() {
      view! { cx, img(src=src) }
    } else {
      view! { cx, figure {
        img(src=src)
        figcaption { (format!("{t}")) }
      }}
    },
    ICreole::Silentbreak => view! { cx, " " },
    ICreole::ForceLinebreak => view! { cx, br },
    ICreole::HorizontalLine => view! { cx, hr },
    ICreole::Table(children) => {
      let t = G::element_from_tag("table");
      let head = &G::element_from_tag("thead");
      let body = &G::element_from_tag("tbody");
      t.append_child(head);
      t.append_child(body);
      for c in children {
        let is_head = if let ICreole::TableHeaderRow(_) = c {
          true
        } else { false };
        if let Some(n) = CreoleItem(cx, c).as_node() {
          if is_head {
            head
          } else {
            body
          }.append_child(n);
        }
      }
      View::new_node(t)
    },
    ICreole::TableHeaderRow(children) | ICreole::TableRow(children) => creole_as_node(cx, "tr", children),
    ICreole::TableHeaderCell(children) | ICreole::TableCell(children) => creole_as_node(cx, "td", children),
    ICreole::BulletList(children) => creole_as_node(cx, "ul", children),
    ICreole::NumberedList(children) => creole_as_node(cx, "ol", children),
    ICreole::ListItem(children) => creole_as_node(cx, "li", children),
    _ => view! { cx, span }
  }
}

#[derive(Prop)]
struct CreolePreviewProps<'a> {
  value: &'a ReadSignal<Box<str>>,
  // parsed: &'a ReadSignal<Vec<ICreole<'a>>>,
  // value: Vec<ICreole<'a>>,
}
#[component]
fn CreolePreview<'a, G: Html>(cx: Scope<'a>, props: CreolePreviewProps<'a>) -> View<G> {
  let vp = create_memo(cx, move || create_ref(cx, props.value.get().clone()));
  let parsed = create_memo(cx, || {
    let window = web_sys::window().expect("no global `window` exists");
    let perf = window.performance();
    if let Some(perf) = &perf {
      perf.mark("s2").unwrap_or(());
    }
    let rst = creoles(*vp.get());
    if let Some(perf) = &perf {
      perf.mark("e2").unwrap_or(());
      measure(perf, "creole parse&render", "s2", "e2");
    }
    debug!("parsed  : {:?}", rst);
    rst
  });

  view!{ cx,
    div(class="preview") {
      h2 { "Preview" }
      Indexed {
        iterable: parsed,
        view: |cx, x: ICreole| view! { cx, CreoleItem(x) }
      }
    }
  }
}
#[derive(Prop)]
struct CreoleEditorProps<'a> {
  value: &'a Signal<Box<str>>,
  // parsed: &'a Signal<Vec<ICreole<'a>>>,
}

#[component]
fn CreoleEditor<'a, G: Html>(cx: Scope<'a>, props: CreoleEditorProps<'a>) -> View<G> {
  let node_ref = create_node_ref(cx);
  let window = web_sys::window().expect("no global `window` exists");
  let local_storage = window.local_storage().unwrap().expect("user has not enabled localStorage"); 

  let last_update = create_signal(cx, 0.);
  let updated = create_signal(cx, false);
  
  {
    spawn_local_scoped(cx, async move {
      loop {
        TimeoutFuture::new(500).await;

        let sec_ago = Date::now() - 1000.;
        if *updated.get()
          || *last_update.get() >= sec_ago
          { continue }
        let node = node_ref.get::<DomNode>();
        let e: HtmlTextAreaElement = node.unchecked_into();

        let perf = window.performance();
        if let Some(perf) = &perf {
          perf.clear_marks();
          perf.clear_measures();
          perf.mark("s1").unwrap_or(());
        }
        let s = e.value();
        // debug!("input : {}", s);
        props.value.set(s.into_boxed_str());

        if let Some(perf) = &perf {
          perf.mark("e1").unwrap_or(());
          measure(perf, "creole input update", "s1", "e1");
        }
        updated.set(true);
        last_update.set(Date::now());
      }
    });
  }

  let on_input = |e: Event| {
    let e: InputEvent = e.unchecked_into();
    updated.set(false);
    debug!("typed : {:?}", e.data());
  };

  view! { cx,
    div(class="editor") {
      h2 { "Editor" }
      textarea(ref=node_ref, /* bind:value=props.value, */ on:input=on_input) {
        "== [[https://webassembly.org|WASM]] [[http://www.wikicreole.org|Creole]] //Live// Editor ([[https://github.com/chidea/wasm-creole-live-editor|github]])
----
=== text styles
//italic// and **bold**.
----
=== bullet list
* a
** aa
*** aaa
* b
----
=== numbered list
# 1
## 11
### 111
# 2
## 21
## 22
----
=== mixed list
* a
*# a1
*# a2
*## a21
#### 1111
#### 1112
### 112
##* 11a
##* 11b
##*# 11b1
##*# 11b2
* a
----
=== links
[[https://www.w3schools.com/]]

[[https://webassembly.org]]

[[http://www.wikicreole.org]]

[[javascript:alert('hi')|alert me \"hi\"]]

[[/|reload to test autosave]]
----
=== headings
== h1
=== h2
==== h3
===== h4
====== h5
======= h6
----
=== linebreaks
No
linebreak!

Use empty row

Force\\\\linebreak
----
=== Horizontal line
----
----
=== image
{{http://www.wikicreole.org/imageServlet?page=CheatSheet%2Fcreole_cheat_sheet.png&width=340}}

{{https://www.w3schools.com/html/w3schools.jpg}}
{{https://www.w3schools.com/html/w3schools.jpg|w3schools}}
----
=== table
|=|=table|=header|
|a|table|row|
|b|table|row|
|c||empty cell|
=== Don't format
{{{
== [[Nowiki]]:
//**don't** format//
}}}
"
      }
    }
  }
}

#[derive(Prop)]
struct CreoleProps {
  editable: bool,
  path: String,
}

#[component]
fn Creole<G: Html>(cx: Scope, props: CreoleProps) -> View<G> {
  let value = create_signal(cx, String::new().into_boxed_str());
  if props.editable {
    view! { cx,
      div(class="wrapper") {
        CreoleEditor {
          value: value,
        }
        CreolePreview{ value :value }
      }
    }
  } else {
    view! { cx,
      CreolePreview{ value : value }
    }
  }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
  view! { cx,
    nav {
      a(href="/") { ("Home(test editor)") }
      a(href="/help") { ("Help") }
    }
    Router {
      integration: HistoryIntegration::new(),
      view: |cx, route: &ReadSignal<AppRoutes>| {
        view! { cx,
          div(class="app") {
            (match route.get().as_ref() {
              AppRoutes::Index => {
                let editable = create_signal(cx, true);
                view! { cx,
                  Creole{ editable: true, path: String::new() }
                }
              },
              AppRoutes::Help => {
                view! { cx,
                  Creole { editable: false, path: String::new() }
                }
              },
              AppRoutes::Wiki{path} => {
                view! { cx,
                  Creole { editable: false, path: path.join("/") }
                }
              },
              AppRoutes::NotFound => view! { cx,
                "404 Not Found"
              },
            })
          }
        }
      }
    }
  }
}

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Debug).unwrap();

  sycamore::render(|cx| component(|| App(cx, ())));
}