use {
    creole_nom::prelude::{Creoles, Creole},
    // log::*,
    serde::{Deserialize, Serialize},
    yew::{
        prelude::*,
        services::storage::{Area, StorageService},
        virtual_dom::VNode,
    },
    // web_sys::{Performance, window, },
};

pub const KEY: &str = "yew.wasm-creole-live-editor.value";

pub struct CreoleLiveEditor {
    link: ComponentLink<Self>,
    key: String,
    storage: StorageService,
    props: Props,
    // state: State,
    // #[cfg(feature = "console_log")]
    // performance: Performance,
    // parsed: Creoles,
}

// #[derive(Serialize, Deserialize, Default)]
// pub struct State {
//     parsed: Vec<Creole>,
//     // /// now as f64 for performance timer
//     // now: f64,
// }

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Props {
    /// used for auto save. leave it default, blank, unspecified to disable autosave.
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    /// by default it's true thus shows textarea as editor. When it's set to false, it's preview-only.
    #[prop_or(true)]
    pub editable: bool,
    /// editor should be autofocused
    #[prop_or(true)]
    pub autofocus: bool,
}

pub enum Msg {
    Edit(String),
    Nope,
}

impl CreoleLiveEditor {
    pub fn get_save_key(name: &str) -> String {
        format!("{}.{}", KEY, name)
    }
}

impl Component for CreoleLiveEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        // let performance = window().unwrap().performance().unwrap();
        let key = if props.name != "" {
            Self::get_save_key(&props.name)
        } else {
            String::new()
        };
        let props = {
            if key != "" && props.value == "" {
                match storage.restore(&key) {
                    Ok(p) => Props{ value:p, ..props }, // Props::builder().value(p).editable(props.editable).autofocus(props.autofocus).build(),
                    _ => props, //Props::builder().editable(props.editable).autofocus(props.autofocus).build(),
                }
            } else { props }
        };
        // let state = if props.value == "" {
        //     State::default()
        // } else {
        //     State {
        //         parsed: props.value.parse().unwrap(),
        //         // now : performance.now(),
        //         ..Default::default()
        //     }
        // };
        // let parsed = props.value.parse().unwrap();
        Self {
            link,
            props,
            storage,
            key,
            // parsed,
            // state,
            // performance,
        }
    }

    // fn rendered(&mut self, first_render: bool) {
        // let dt = self.performance.now() - self.state.now;
        // let render_type = if first_render { "first " } else { "" };
        // #[cfg(all(feature = "console_log", feature = "performance"))]
        // info!("{}render done for : {}ms", render_type, dt);
    // }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let b = self.props != props;

        if !b {
            if self.props.name != props.name {
                self.key = format!("{}.{}", KEY, self.props.name);
            }
            self.props = props;
        }
        b
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(val) => {
                // if !self.props.editable {
                //     return false;
                // }
                // self.state.now = self.performance.now();

                self.props.value = val;
                // self.parsed = self.props.value.parse().unwrap();
                if self.key != "" {
                    self.storage.store(&self.key, Ok(self.props.value.clone()));
                }
                true
            }
            Msg::Nope => {false}
        }
    }

    fn view(&self) -> Html {
        if self.props.editable {
            html! {
            <div class="creole-live-editor--wrapper">
                { self.view_input() }
                { self.view_preview() }
            </div> }
        }else {
            self.view_preview()
        }
    }
}

impl CreoleLiveEditor {
    fn view_input(&self) -> Html {
        if !self.props.editable {
            return html! {};
        }
        html! {
            <textarea class="creole-live-editor--textarea"
                autofocus=self.props.autofocus
                value=&self.props.value // propagates from parent value property
                oninput=self.link.callback(|e: InputData| Msg::Edit(e.value)) />
        }
    }

    fn view_preview(&self) -> Html {
        let parsed :Creoles = self.props.value.parse().unwrap();
        html! {
            <div class="creole-live-editor--preview">
                { for parsed.iter().map(|i| render_creole(i)) }
            </div>
        }
    }
}

// impl State {
    fn render_creole(c: &Creole) -> VNode {
        match &c {
            Creole::Bold(b) => html! {<b>{b}</b>},
            Creole::Italic(b) => html! {<i>{b}</i>},
            Creole::Text(b) => html! {<span>{b}</span>},
            Creole::BulletList(i, b) => match i {
                0 => html! {<ul><li>{b}</li></ul>},
                1 => html! {<ul><ul><li>{b}</li></ul></ul>},
                2 => html! {<ul><ul><ul><li>{b}</li></ul></ul></ul>},
                3 => html! {<ul><ul><ul><ul><li>{b}</li></ul></ul></ul></ul>},
                _ => html! {<ul><ul><ul><ul><ul><li>{b}</li></ul></ul></ul></ul></ul>},
            }
            Creole::NumberedList(i, b) => match i {
                0 => html! {<ol><li>{b}</li></ol>},
                1 => html! {<ol><ol><li>{b}</li></ol></ol>},
                2 => html! {<ol><ol><ol><li>{b}</li></ol></ol></ol>},
                3 => html! {<ol><ol><ol><ol><li>{b}</li></ol></ol></ol></ol>},
                _ => html! {<ol><ol><ol><ol><ol><li>{b}</li></ol></ol></ol></ol></ol>},
            }
            Creole::Heading(i, b) => match i {
                0 => html! {<h1>{b}</h1>},
                1 => html! {<h2>{b}</h2>},
                2 => html! {<h3>{b}</h3>},
                3 => html! {<h4>{b}</h4>},
                4 => html! {<h5>{b}</h5>},
                _ => html! {<h6>{b}</h6>},
            }
            Creole::HorizontalLine => html! {<hr />},
            Creole::Linebreak => html! {<br />},
            Creole::Link(link, name) => {
                let link = link.as_str();
                let name = if name.is_empty() { link } else { &name };
                html! {<a href=link>{name}</a>}
            }
            Creole::Image(link, name) => {
                let (img, label) = (html!{<img src=link.as_str() />}, name.as_str());
                if label.is_empty() { img }
                else { html!{<div class="creole-live-editor--img-wrapper">{img}<br /><span>{label}</span></div>} }
            }
            _ => html! {}
        }
    }
// }