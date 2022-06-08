use {
    crate::cle::CreoleLiveEditor,
    yew::{
        prelude::*,
        // services::storage::{Area, StorageService},
    },
};

pub struct App{
    // link: ComponentLink<Self>,
    saved_value: String,
}
pub enum Msg{
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // web_sys::window().unwrap().navigator().service_worker().register("sw.js");
        let storage = StorageService::new(Area::Local).unwrap();
        let key = CreoleLiveEditor::get_save_key("editor1");
        let saved_value = match storage.restore(&key) {
            Ok(v) => v,
            _ => String::new()
        };

        Self { /* link,  */saved_value, }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let value = if self.saved_value == "" {"== WASM Creole Live editor
----
=== headings
== h1
=== h2
==== h3
----
=== text styles
//italic// and **bold**.

linebreak1\\\\linebreak2
----
=== unordered list
* a
** b
*** c
----
=== ordered list
# a
## b
### c
----
=== images
{{https://www.w3schools.com/html/w3schools.jpg}} 
{{https://www.w3schools.com/html/w3schools.jpg|w3schools}}
----
=== links
[[https://www.w3schools.com/]]

[[javascript:alert('hi')|alert me \"hi\"]]

[[/|reload to test autosave]]"} else { &self.saved_value };

        html! { <>
            <div class="wrapper">
                <h2>{"WASM Creole Live Editor example"}</h2>
                <CreoleLiveEditor name="editor1" value=value />
            </div>
            <div class="wrapper">
                <h2>{"Preview-only mode (last saved content of editor above)"}</h2>
                <CreoleLiveEditor name="editor1" editable=false />
            </div>
        </>}
    }
}