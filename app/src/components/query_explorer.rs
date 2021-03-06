use super::query_form::QueryForm;
use super::query_result::QueryResult;
use super::query_toolbar::QueryToolbar;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/api.js")]
extern "C" {
    fn getResult() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct QueryExplorer {
    link: ComponentLink<Self>,
    run_count: i32,
    query_result: Option<String>,
}

pub enum Msg {
    RunQuery,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for QueryExplorer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryExplorer {
            link,
            run_count: 0,
            query_result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunQuery => {
                self.run_count += 1;
                self.query_result = Some(getResult());
                log(self.query_result.clone().unwrap_or_default().as_str());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-explorer">
                <QueryForm region="ap-southeast-2" table_name="cloudbin" />
                <div class="query-result-pane">
                    <QueryToolbar run_count={self.run_count} on_run_clicked=self.link.callback(|_| Msg::RunQuery) />
                    <QueryResult result={self.query_result.clone()} />
                </div>
            </div>
        }
    }
}
