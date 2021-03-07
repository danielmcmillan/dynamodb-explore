use super::query_form::QueryForm;
use super::query_result::QueryResult;
use super::query_toolbar::QueryToolbar;
use crate::dynamodb;
use yew::prelude::*;

pub struct QueryExplorer {
    link: ComponentLink<Self>,
    query_result: Option<dynamodb::ResultData>,
    on_run_query: Callback<()>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub query_result: Option<dynamodb::ResultData>,
    pub on_run_query: Callback<()>,
}

impl Component for QueryExplorer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryExplorer {
            link,
            query_result: props.query_result,
            on_run_query: props.on_run_query,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.query_result = props.query_result;
        self.on_run_query = props.on_run_query;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-explorer">
                <QueryForm region="ap-southeast-2" table_name="cloudbin" />
                <div class="query-result-pane">
                    <QueryToolbar on_run_clicked=self.on_run_query.clone() />
                    <QueryResult result={self.query_result.clone()} />
                </div>
            </div>
        }
    }
}
