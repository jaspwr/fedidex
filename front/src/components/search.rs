use yew::prelude::*;

pub struct SearchBox {
    props: Props,
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
}



impl Component for SearchBox {
    type Message = ();
    type Properties = Props;


    fn create(ctx: &Context<Self>) -> Self {
        Self { props: Props { value: "".to_string()} }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <span id="search-label">
                    {"Search:"}
                </span>
                <input type="text" class="search"/>
            </>
        }
    }
}