use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Instance {
    pub name: String,
    pub url: String,
    pub users: u32,
    pub invite_only: bool,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub instance: Instance,
}

#[function_component(InstanceListItem)]
pub fn inst_item_list(props: &Props) -> Html {
    html! {
        <>
            <div class="instance-list-item favicon"></div>
            <div class="instance-list-item name">
                <a href={format!("https://{}", &props.instance.name)}>
                    {&props.instance.name}
                </a>
            </div>
            <div class="instance-list-item description">{"desc"}</div>
            <div class="instance-list-item invite-only">{"asdf"}</div>
            <div class="instance-list-item users">{3}</div>
            <div class="instance-list-item type">{"Mastodon"}</div>
        </>
    }
}