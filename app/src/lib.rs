mod components;
mod dynamodb;
mod utils;
use self::components::query_explorer::QueryExplorer;
use self::utils::set_panic_hook;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    link: ComponentLink<Self>,
    query_result: Option<dynamodb::ResultData>,
}

pub enum Msg {
    RunQuery,
    QueryStarted,
    QueryComplete(dynamodb::ResultData),
    QueryFailed(dynamodb::DynamoDBRequestFailed),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            query_result: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunQuery => {
                let link = self.link.clone();
                let mut values = HashMap::new();
                values.insert(String::from(":pk"), String::from("BEhS_eOqywMCJaA="));
                let request = dynamodb::Input {
                    profile: String::from("default"),
                    region: String::from("ap-southeast-2"),
                    table_name: String::from("cloud-chat-dev"),
                    // request_type: dynamodb::RequestType::Query(dynamodb::QueryInput {
                    //     key_condition_expression: String::from("pk = :pk"),
                    // }),
                    request_type: dynamodb::RequestType::Scan,
                    expression_attribute_names: None,
                    expression_attribute_values: Some(values),
                };
                spawn_local(async move {
                    match dynamodb::request(&request).await {
                        Ok(data) => link.send_message(Msg::QueryComplete(data)),
                        Err(err) => link.send_message(Msg::QueryFailed(err)),
                    }
                });
                self.link.send_message(Msg::QueryStarted);
                false
            }
            Msg::QueryStarted => {
                let mut values = HashMap::new();
                values.insert(String::from("loading"), String::from("..."));
                self.query_result = Some(vec![values]);
                true
            }
            Msg::QueryComplete(result) => {
                self.query_result = Some(result);
                true
            }
            Msg::QueryFailed(err) => {
                let mut values = HashMap::new();
                values.insert(String::from("error"), String::from(err.to_string()));
                self.query_result = Some(vec![values]);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <QueryExplorer query_result=self.query_result.clone() on_run_query=self.link.callback(|_| Msg::RunQuery) />
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    set_panic_hook();
    App::<Model>::new().mount_to_body();
}
