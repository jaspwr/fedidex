use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub number: u32,
}

#[function_component(PrettyNumber)]
pub fn pretty_number(props: &Props) -> Html {
    html! {
        <>
        {
            if props.number > 1000 {
                html! { <>{format!("{:.1}k", props.number / 1000)}</> }
            } else {
                html! { <>{props.number}</> }
            }
        }
        </>
    }
}