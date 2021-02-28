// use super::query_form::QueryForm;
use yew::prelude::*;
use yew::html;
use yew_material::MatButton;

pub struct QueryToolbar {
    link: ComponentLink<Self>,
    on_run_clicked: Callback<()>,
    run_count: i32,
}

pub enum Msg {
    RunClicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_run_clicked: Callback<()>,
    pub run_count: i32,
}

impl Component for QueryToolbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryToolbar {
            link,
            on_run_clicked: props.on_run_clicked,
            run_count: props.run_count,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunClicked => self.on_run_clicked.emit(()),
        };
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.run_count = props.run_count;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-toolbar">
                <span onclick=self.link.callback(|_| Msg::RunClicked)>
                    <MatButton label=format!("Run {}", self.run_count) />
                </span>
            </div>
        }
    }
}
