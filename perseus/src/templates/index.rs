use perseus::{Html, RenderFnResultWithCause, SsrNode, Template, web_log};
use sycamore::{
  prelude::{view, cloned, View, Signal, create_effect, create_memo, create_selector, },
  context::{ContextProvider, ContextProviderProps, use_context},
  noderef::NodeRef,
  generic_node::dom_node::DomNode,
  builder::prelude::*,
};
use creole_nom::prelude::{ICreole, creoles, try_creoles };

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    let preview_ref = NodeRef::new();
    let text_ref = NodeRef::new();
    let text = Signal::new(String::new());
    // let parsed = create_memo(cloned!(text => move || creoles(&text.get()) ));

    create_effect(cloned!(state => move || {
      println!("New text : {}", text.get());
    }));
    // let pr = preview_ref.clone();
    view! { 
      textarea(ref=text_ref, on:keypress=|e| {
        web_log!("key pressed : {:?}", e);
        // if let Some(r) = pr.try_get::<DomNode>(){
        // }
        // text.set(e.)
      }) {}
      button(on:click=|_| {
      }) {
        "Click me"
      }
      p { (state.greeting.get()) }
      div (ref=preview_ref) {}
      a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
  Template::new("index")
      .build_state_fn(get_build_state)
      .template(index_page)
      .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
  view! {
    title { "Index Page | Perseus Example – Basic" }
  }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
  _path: String,
  _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
  Ok(IndexPageState {
    greeting: "Hello World!".to_string(),
  })
}