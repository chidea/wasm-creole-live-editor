#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use log::debug;

use gloo_timers::future::TimeoutFuture;
use js_sys::{Date, };
use sycamore::{/* builder::prelude::*,  */futures::spawn_local_scoped, suspense::Suspense, prelude::*};
use sycamore_router::{HistoryIntegration, Router};
use wasm_bindgen::{JsCast, closure::Closure, prelude::*};
use wasm_bindgen_futures::JsFuture;

use web_sys::{Event, /* HtmlInputElement,  */ HtmlTextAreaElement, InputEvent, IdbDatabase, IdbOpenDbRequest, IdbObjectStore};

use creole_nom::prelude::*;

mod route;
use route::AppRoutes;

use urlencoding::decode;

// TODO : light/dark theme
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

fn measure(perf: &web_sys::Performance, name: &str, s: &str, e: &str) {
    if perf
        .measure_with_start_mark_and_end_mark(name, s, e)
        .is_ok()
    {
        let m: web_sys::PerformanceMeasure = perf
            .get_entries_by_name_with_entry_type(name, "measure")
            .get(0)
            .unchecked_into();
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

// #[derive(Prop)]
// struct CreoleItemProps<'a> {
//     item: &'a ReadSignal<ICreole<'a>>,
// }

#[component]
fn CreoleItem<'a, G: Html>(cx: Scope<'a>, i: ICreole<'a>) -> View<G> {
    match i {
        ICreole::Heading(l, t) => creole_as_node(cx, &format!("h{l}"), t),
        ICreole::Bold(children) => creole_as_node(cx, "b", children),
        ICreole::Italic(children) => creole_as_node(cx, "i", children),
        ICreole::Text(t) => view! { cx, span { (format!("{t}")) } },
        ICreole::DontFormat(t) => view! { cx, pre { (format!("{t}"))  } },
        ICreole::Link(href, t) => {
            if href.starts_with("http://") || href.starts_with("https://") {
                view! { cx, a(href=href, target="__blank") { (format!("{t}")) } }
            } else {
                let path = format!("/w/{}", href);
                let p = path.clone();
                let on_click = move |e: Event| {
                  e.prevent_default();
                  sycamore_router::navigate(&p);
                };
                view! { cx, a(href=path, on:click=on_click) { (format!("{t}")) } }
            }
        }
        ICreole::Line(l) => creole_as_node(cx, "p", l),
        ICreole::Image(src, t) => {
            if t.is_empty() {
                view! { cx, img(src=src) }
            } else {
                view! { cx, figure {
                  img(src=src)
                  figcaption { (format!("{t}")) }
                }}
            }
        }
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
                let is_head = matches!(c, ICreole::TableHeaderRow(_));
                if let Some(n) = CreoleItem(cx, c).as_node() {
                    if is_head { head } else { body }.append_child(n);
                }
            }
            View::new_node(t)
        }
        ICreole::TableHeaderRow(children) | ICreole::TableRow(children) => {
            creole_as_node(cx, "tr", children)
        }
        ICreole::TableHeaderCell(children) | ICreole::TableCell(children) => {
            creole_as_node(cx, "td", children)
        }
        ICreole::BulletList(children) => creole_as_node(cx, "ul", children),
        ICreole::NumberedList(children) => creole_as_node(cx, "ol", children),
        ICreole::ListItem(children) => creole_as_node(cx, "li", children),
    }
}

#[derive(Prop)]
struct CreolePreviewProps<'a> {
    value: &'a ReadSignal<String>,
    show_title: bool,
    // parsed: &'a ReadSignal<Vec<ICreole<'a>>>,
    // value: Vec<ICreole<'a>>,
}
#[component]
fn CreolePreview<'a, G: Html>(cx: Scope<'a>, props: CreolePreviewProps<'a>) -> View<G> {
    let vp = create_memo(cx, move || create_ref(cx, props.value.get()));
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

    view! { cx,
      div(class="preview") {
        ( if props.show_title {
            view!{ cx, h2(class="view-name") { "Preview" } }
          } else {
            view!{ cx, } 
          }
        )
        Indexed {
          iterable: parsed,
          view: |cx, x: ICreole| view! { cx, CreoleItem(x) }
        }
      }
    }
}
#[derive(Prop)]
struct CreoleEditorProps<'a> {
    default: String,
    value: &'a Signal<String>,
}
const HELP : &str = "= Help
== important note
Everything you 'edit' in this app stays in your browser(into IndexedDB).

Using any other device, browser, domain, even protocol or clearing your browser cache will erase every note of yourself.

Browsers may limit or ask you for storage expansion when total saved notes are becoming larger than its maximum.
----
== text styles
//italic// and **bold**.
----
== bullet list
* a
** aa
*** aaa
* b
----
== numbered list
# 1
## 11
### 111
# 2
## 21
## 22
----
== mixed list
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
== external links
[[https://webassembly.org|WASM]]

[[http://www.wikicreole.org|WikiCreole]]

[[https://www.w3schools.com/]]

== script links
[[javascript:alert('hi')|alert me \"hi\"]]

== internal links
[[home]]

[[test]] : view wiki page named \"test\"

----
== headings
= h1
== h2
=== h3
==== h4
===== h5
====== h6
----
== linebreaks
No
linebreak!

Use empty row

Force\\\\linebreak
----
== Horizontal line
----
----
== image
{{http://www.wikicreole.org/imageServlet?page=CheatSheet%2Fcreole_cheat_sheet.png&width=340}}

{{/icons/icon-64.png|WCLEW Logo}}
----
== table
|=|=table|=header|
|a|{{{ // no wiki in table // }}}|row|
|b|table|row|
|c||empty cell|
== Don't format
{{{
== [[Nowiki]]:
//**don't** format//
}}}
";
#[component]
fn CreoleEditor<'a, G: Html>(cx: Scope<'a>, props: CreoleEditorProps<'a>) -> View<G> {
    let node_ref = create_node_ref(cx);

    let last_update = create_signal(cx, 0.);
    let updated = create_signal(cx, false);
    let default_value = props.default;
    {
        let window = web_sys::window().expect("no global `window` exists");
        spawn_local_scoped(cx, async move {
            loop {
                TimeoutFuture::new(500).await;

                let sec_ago = Date::now() - 1000.;
                if *updated.get() || *last_update.get() >= sec_ago {
                    continue;
                }
                let node = node_ref.get::<DomNode>();
                let e: HtmlTextAreaElement = node.unchecked_into();

                let perf = window.performance();
                if let Some(perf) = &perf {
                    perf.clear_marks();
                    perf.clear_measures();
                    perf.mark("s1").unwrap_or(());
                }
                props.value.set(e.value().into());

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
        h2(class="view-name") { "Editor" }
        textarea(ref=node_ref, on:input=on_input) {
          (default_value)
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
async fn Creole<G: Html>(cx: Scope<'_>, props: CreoleProps) -> View<G> {
    let store = open_db_store(cx, props.editable);
    let path = props.path.clone();
    debug!("getting : {}", path);

    let value =
      if path == "help" {
          String::from(HELP)
      } else if let Ok(r) = store.get(&path.into()) {
        JsFuture::from(js_sys::Promise::new(&mut |resolve : js_sys::Function, _reject: js_sys::Function|{
          let cb = Closure::once(move |e: &Event| -> Result<(), JsValue> {
            if let Some(t) = e.target() {
              resolve.call1(&JsValue::NULL,
                &t.unchecked_into::<web_sys::IdbRequest>().result().expect("")
              ).expect("");
            }
            Ok(())
          });
          r.set_onsuccess(Some(create_ref(cx, cb).as_ref().unchecked_ref()));
        })).await.expect("promise").as_string().unwrap_or(String::new())
      } else {
        String::new()
      };

    let value_signal : &Signal<String> = create_signal(cx, value.clone());

    if props.editable {
        let path : JsValue = props.path.clone().into();
        create_effect(cx, move || {
          let store = open_db_store(cx, props.editable);
          let value = &*value_signal.get();
          let path = path.as_string().unwrap();
          debug!("saving to : {}, value : {}", path, value);
          if path != "help" {
            if value.is_empty() {
              if let Ok(_r) = store.delete(&path.into()) {
              }
            } else if let Ok(_r) = store.put_with_key(&value.into(), &path.into()) {
            }
          }
        });
        view! { cx,
          div(class="wrapper") {
            CreoleEditor {
              value: value_signal,
              default: value,
            }
            CreolePreview{ value :value_signal, show_title: true }
          }
        }
    } else {
        view! { cx,
          CreolePreview{ value : value_signal, show_title: false }
        }
    }
}

const DB_NAME: &str = "wiki";
const STORE_NAME: &str = "wiki";

async fn init_db(cx: Scope<'_>) -> IdbDatabase {
    JsFuture::from(js_sys::Promise::new(&mut |resolve : js_sys::Function, _reject: js_sys::Function|{
        let window = web_sys::window().expect("no global `window` exists");
        let req = window
            .indexed_db().unwrap().expect("user has not enabled IndexedDB")
            .open(DB_NAME).expect("wiki DB is not available");
        let a = Closure::once(move |evt: &web_sys::IdbVersionChangeEvent| -> Result<(), JsValue> {
          if let Some(t) = evt.target() {
            let wiki_db = t.unchecked_into::<IdbOpenDbRequest>().result().expect("")
              .unchecked_into::<IdbDatabase>();
            let wiki_store = wiki_db.create_object_store(STORE_NAME)?;
            wiki_store.put_with_key(&HELP.into(), &"".into())?;
          }
          Ok(())
        });
        req.set_onupgradeneeded(Some(a.as_ref().unchecked_ref()));
        a.forget();
        let b = Closure::once(move |e: &Event| -> Result<(), JsValue> {
          if let Some(t) = e.target() {
            resolve.call1(&JsValue::NULL, &t.unchecked_into::<IdbOpenDbRequest>().result().expect("")).expect("");
          }
          Ok(())
        });
        req.set_onsuccess(Some(create_ref(cx, b).as_ref().unchecked_ref()));
    })).await.expect("promise").unchecked_into()
}
fn open_db_store(cx: Scope, write: bool) -> IdbObjectStore {
    let db = use_context::<RcSignal<IdbDatabase>>(cx);
    let transaction = db.get().transaction_with_str_and_mode(STORE_NAME, 
      if write { web_sys::IdbTransactionMode::Readwrite }
      else { web_sys::IdbTransactionMode::Readonly }).expect("could not open transaction");
    transaction.object_store(STORE_NAME).expect("could not open store")
}
#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let db = create_rc_signal(init_db(cx).await);
    debug!("db opened : {:?}, name : {}", db, db.get().name());
    provide_context(cx, db);

    let wiki_path_node_ref = create_node_ref(cx);
    let wiki_path = create_signal(cx, String::new());

    let set_wiki_path = |s: String| {
        wiki_path.set(s.clone());
        s
    };

    view! { cx,
      nav {
        button(on:click=|_| sycamore_router::navigate("/w/")) { ("Home view") }
        button(on:click=|_| sycamore_router::navigate("/e/")) { ("Home edit") }
        button(on:click=|_|sycamore_router::navigate("/help")) { ("Help") }
        input(type="text", bind:value=wiki_path, ref=wiki_path_node_ref) { }
        button(on:click=|_| sycamore_router::navigate(&format!("/e/{}", *wiki_path.get()))){ ("Edit") }
        button(on:click=|_| sycamore_router::navigate(&format!("/w/{}", *wiki_path.get()))){ ("View") }
        button(on:click=|_| sycamore_router::navigate(&format!("/d/{}", *wiki_path.get()))){ ("Delete") }
      }
      Router {
        integration: HistoryIntegration::new(),
        view: move |cx, route: &ReadSignal<AppRoutes>| {
          use AppRoutes::*;
          view! { cx,
            div(class="app") {
              Suspense {
                fallback: view!{ cx, "Reading from DB..."},
                (match route.get().as_ref() {
                  Index => {
                    view! { cx,
                      Creole{ editable: false, path: set_wiki_path(String::new()) }
                    }
                  },
                  Help => {
                    view! { cx,
                      Creole { editable: true, path: String::from("help") }
                    }
                  },
                  Wiki{path} => {
                    let path = path.into_iter().map(|s: &String| decode(s).expect("UTF8").into_owned()).collect::<Vec<String>>();
                      // decode(&path.join("/")).expect("UTF8").into_owned();
                    view! { cx,
                      Creole { editable: false, path: set_wiki_path(path.join("/")) }
                    }
                  },
                  WikiEdit{path} => {
                    let path = path.into_iter().map(|s: &String| decode(s).expect("UTF8").into_owned()).collect::<Vec<String>>();
                    view! { cx,
                      Creole { editable: true, path: set_wiki_path(path.join("/")) }
                    }
                  },
                  WikiDelete{path} => {
                    let path = path.into_iter().map(|s: &String| decode(s).expect("UTF8").into_owned()).collect::<Vec<String>>();
                    let p = path.join("/");
                    wiki_path.set(p.clone());
                    let pp : std::cell::RefCell<JsValue> = std::cell::RefCell::new(p.clone().into());
                    let on_del_yes = move |_| {
                      let store = open_db_store(cx, true);
                      if let Ok(_) = store.delete(&pp.take()){}
                      sycamore_router::navigate("/");
                    };
                    view! { cx,
                      p {
                        h2 { "delete?" }
                        button(on:click=on_del_yes) { ("Yes")}
                      }
                      Creole { editable: false, path: p }
                    }
                  },
                  NotFound => view! { cx,
                    "404 Not Found"
                  },
                })
              }
            }
          }
        }
      }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    // sycamore::render(|cx| component(|| App(cx, ())));
    sycamore::render(|cx| view! { cx,
      Suspense {
        fallback: view!{ cx, "Opening DB..."},
        App {}
      }
    });
}
