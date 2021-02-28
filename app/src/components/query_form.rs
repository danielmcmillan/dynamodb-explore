use yew::prelude::*;
use yew_material::MatTextField;

pub struct QueryForm {
    link: ComponentLink<Self>,
    region: String,
    table_name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub region: String,
    pub table_name: String,
}

impl Component for QueryForm {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QueryForm {
            link,
            region: props.region,
            table_name: props.table_name,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.region = props.region;
        self.table_name = props.table_name;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-form">
                <MatTextField label="Region" value=&self.region />
                <MatTextField label="Table" value=&self.table_name />
            </div>
        }
    }
}
