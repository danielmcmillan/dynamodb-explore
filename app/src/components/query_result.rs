use yew::prelude::*;

pub struct QueryResult {
    link: ComponentLink<Self>,
    result: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub result: Option<String>,
}

impl Component for QueryResult {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryResult {
            link,
            result: props.result,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.result = props.result;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-result">
                {"Result: "}{self.result.clone().unwrap_or(String::from("None"))}
            </div>
        }
    }
}
