use {
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

const KEY: &str = "yew.wasm-creole-live-editor.value";

pub struct CreoleLiveEditor {
    link: ComponentLink<Self>,
    key: String,
    storage: StorageService,
    state: State,
    props: Props,
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    parsed: Vec<Creole>,
}

#[derive(Serialize, Deserialize, Clone, Properties)]
pub struct Props {
    #[prop_or_default()]
    /// used for auto save. leave it default, blank, unspecified to disable autosave.
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or(true)]
    /// by default it's true thus shows textarea as editor. When it's set to false, it's preview-only.
    pub editable: bool,
    #[prop_or_default]
    pub style: String,
}

pub enum Msg {
    Edit(String),
    Nope,
}

impl Component for CreoleLiveEditor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let key = if props.name != "" {
            format!("{}.{}", KEY, props.name)
        } else {
            String::new()
        };
        let props = {
            if key != "" && props.value == "" && props.editable {
                match storage.restore(&key) {
                    Ok(p) => Props::builder().value(p).build(),
                    _ => Props::builder().build(),
                }
            } else {
                props
            }
        };
        let state = if props.value == "" {
            State::default()
        } else {
            State {
                parsed: match creoles(&props.value) {
                    Ok((_, v)) => v,
                    _ => vec![],
                },
                ..Default::default()
            }
        };
        Self {
            link,
            props,
            storage,
            key,
            state,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let p = self.props.clone();
        self.props = props;

        if self.props.name != p.name {
            self.key = format!("{}.{}", KEY, self.props.name);
            return true;
        }

        self.props.value != p.value || self.props.editable != p.editable
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Edit(val) => {
                if !self.props.editable {
                    return false;
                }

                self.props.value = val;
                match creoles(&self.props.value) {
                    Ok((_, v)) => {
                        self.state.parsed = v;
                    }
                    _ => (),
                }
                if self.key != "" {
                    self.storage.store(&self.key, Ok(self.props.value.clone()));
                }
            }
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
        <div class="creole-live-editor--wrapper">
          { self.view_input() }
          { self.view_preview() }
        </div> }
    }
}

impl CreoleLiveEditor {
    fn view_input(&self) -> Html {
        if !self.props.editable {
            return html! {};
        }
        html! {
            <textarea class="creole-live-editor--textarea"
                  value=&self.props.value // propagates from parent value property
                  oninput=self.link.callback(|e: InputData| Msg::Edit(e.value)) />
        }
    }

    fn view_preview(&self) -> Html {
        html! {
            <div class="creole-live-editor--preview">
                { self.state.render_creoles_to_html() }
            </div>
        }
    }
}

impl State {
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
            },
            Creole::NumberedList(i, b) => match i {
                0 => html! {<ol><li>{b}</li></ol>},
                1 => html! {<ol><ol><li>{b}</li></ol></ol>},
                2 => html! {<ol><ol><ol><li>{b}</li></ol></ol></ol>},
                3 => html! {<ol><ol><ol><ol><li>{b}</li></ol></ol></ol></ol>},
                _ => html! {<ol><ol><ol><ol><ol><li>{b}</li></ol></ol></ol></ol></ol>},
            },
            Creole::Heading(i, b) => match i {
                0 => html! {<h1>{b}</h1>},
                1 => html! {<h2>{b}</h2>},
                2 => html! {<h3>{b}</h3>},
                3 => html! {<h4>{b}</h4>},
                4 => html! {<h5>{b}</h5>},
                _ => html! {<h6>{b}</h6>},
            },
            Creole::HorizontalLine => html! {<hr />},
            Creole::Linebreak => html! {<br />},
            Creole::Link(a, b) => html! {<a href=a.as_str()>{b}</a>},
            _ => html! {},
        }
    }
    fn render_creoles_to_html(&self) -> VNode {
        html! {
            { for self.parsed.iter().map(|i| Self::render_creole(i)) }
        }
    }
}
