use {
    crate::cle::CreoleLiveEditor,
    anyhow::{anyhow, Error},
    creole_nom::prelude::*,
    log::*,
    serde::{Deserialize, Serialize},
    strum::IntoEnumIterator,
    strum_macros::{EnumIter, ToString},
    yew::{
        prelude::*,
        services::storage::{Area, StorageService},
        virtual_dom::VNode,
    },
};

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let value = "== WASM Creole Live editor
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
== list
* a
** b
*** c
----
== numbered list
# a
## b
### c
----
";

        let preview_value = "== Non-editable preview
editable=false option makes it draw only its preview from given value.

Also, it's auto-save feature gets disabled.";

        html! { <>
            <div class="wrapper">
                <h1>{"WASM Creole Live Editor example"}</h1>
                <CreoleLiveEditor name="editor1" value=value />
            </div>
            <div class="wrapper">
                <h1>{"Preview-only mode"}</h1>
                <CreoleLiveEditor value=preview_value editable=false />
            </div>
        </>}
    }
}
