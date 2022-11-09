use crate::instance_list_item::Instance;
use crate::instance_list_item::InstanceListItem;

use yew::prelude::*;

pub struct InstanceList {
    props: Props,
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub items: [Instance; 3]
}


impl Component for InstanceList {
    type Message = ();
    type Properties = Props;


    fn create(ctx: &Context<Self>) -> Self {
        let i = Instance { 
            name: String::from("test"),
            url: String::from("test"),
            users: 0,
            invite_only: true,
        };
        Self { props: Props { items: [i.clone(), i.clone(), i.clone()] }}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="instance-list">
                <div class="instance-list-item-container">
                        <div class="instance-list-item favicon label"></div>
                        <div class="instance-list-item name label">{"Domain"}</div>
                        <div class="instance-list-item description label">{"Description"}</div>
                        <div class="instance-list-item invite-only label">{"Open"}</div>
                        <div class="instance-list-item users label">{"Users"}</div>
                        <div class="instance-list-item type label">{"Platform"}</div>

                    {for self.props.items.iter().map(|item| {
                            html! {
                                < InstanceListItem instance={item.clone()} />
                            }
                        })
                    }
                </div>
            </div>
        }
    }
}