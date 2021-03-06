// use super::query_form::QueryForm;
use yew::html;
use yew::prelude::*;
use yew_material::MatButton;

pub struct QueryToolbar {
    link: ComponentLink<Self>,
    on_run_clicked: Callback<()>,
}

pub enum Msg {
    RunClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_run_clicked: Callback<()>,
}

impl Component for QueryToolbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryToolbar {
            link,
            on_run_clicked: props.on_run_clicked,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunClicked => self.on_run_clicked.emit(()),
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-toolbar">
                <span onclick=self.link.callback(|_| Msg::RunClicked)>
                    <MatButton label="Run" />
                </span>
            </div>
        }
    }
}
