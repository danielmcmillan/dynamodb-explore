mod components;
mod dynamodb;
mod utils;
use self::components::query_explorer::QueryExplorer;
use self::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
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
