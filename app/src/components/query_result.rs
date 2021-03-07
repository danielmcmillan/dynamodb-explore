use crate::dynamodb;
use std::collections::HashMap;
use yew::prelude::*;

pub struct QueryResult {
    link: ComponentLink<Self>,
    result: Option<dynamodb::ResultData>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub result: Option<dynamodb::ResultData>,
}

impl QueryResult {
    fn render_result(result: &dynamodb::ResultData) -> Html {
        html! {
            <ul>
                {result.iter().map(|item| QueryResult::render_item(item)).collect::<Html>()}
            </ul>
        }
    }

    fn render_item(item: &HashMap<String, String>) -> Html {
        html! {
            <li>
                <ul>
                    {item.iter().map(|(k, v)| html! {<li>{k}{": "} {v}</li>}).collect::<Html>()}
                </ul>
            </li>
        }
    }
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
                {
                    if let Some(result) = &self.result {
                        QueryResult::render_result(result)
                    } else {
                        html!{String::from("No results")}
                    }
                }
            </div>
        }
    }
}
