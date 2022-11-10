use crate::instance_list_item::InstanceListItem;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub items: [String; 3]
}

#[function_component(InstanceList)]
pub fn instance_list(props: &Props) -> Html {
    html! {
        <div class="instance-list">
            <div class="instance-list-item-container">
                    <div class="instance-list-item favicon label"></div>
                    <div class="instance-list-item name label">{"Domain"}</div>
                    <div class="instance-list-item description label">{"Description"}</div>
                    <div class="instance-list-item invite-only label">{"Open"}</div>
                    <div class="instance-list-item users label">{"Users"}</div>
                    <div class="instance-list-item type label">{"Platform"}</div>

                {for props.items.iter().map(|item| {
                        html! {
                            < InstanceListItem instance_json={ item.clone() } />
                        }
                    })
                }
            </div>
        </div>
    }
}