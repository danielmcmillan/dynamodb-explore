mod components;
mod dynamodb;
mod utils;
use self::components::query_explorer::QueryExplorer;
use self::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    link: ComponentLink<Self>,
    query_result: Option<String>,
}

pub enum Msg {
    RunQuery,
    QueryComplete(String),
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
                spawn_local(async move {
                    let result = dynamodb::request().await;
                    link.send_message(Msg::QueryComplete(result));
                });
                false
            }
            Msg::QueryComplete(result) => {
                self.query_result = Some(result);
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
