use crate::instance_list_item::InstanceListItem;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub items: Vec<String>,
}

#[function_component(InstanceList)]
pub fn instance_list(props: &Props) -> Html {
    html! {
        <>
            {for props.items.iter().map(|item| {
                    html! {
                        < InstanceListItem instance_json={ item.clone() } />
                    }
                })
            }
        </>
    }
}