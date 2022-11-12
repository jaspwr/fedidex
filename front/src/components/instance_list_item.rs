use yew::prelude::*;
use crate::instance::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub instance_json: String,
}

#[function_component(InstanceListItem)]
pub fn inst_item_list(props: &Props) -> Html {
    let _meta = serde_json::from_str(&props.instance_json);
    if _meta.is_err() {
        return html! {
            <>
            </>
        };
    }

    let meta: ServiceMeta = _meta.unwrap();
    let open_status = if meta.invite_only { "Apply" } else { "Open" };
    html! {
        <>
            <div class="instance-list-item favicon">
                <img class="favicon" src={meta.favicon} alt="icon"/>
            </div>
            <div class="instance-list-item name">
                <a href={format!("https://{}", &meta.name)}>
                    {&meta.name}
                </a>
            </div>
            <div class="instance-list-item description">
                {&meta.description}
            </div>
            <div class="instance-list-item invite-only">
                <span class={format!("openstatus {}", open_status)}>
                    {open_status}
                </span>    
            </div>
            <div class="instance-list-item users">
                {&meta.users}
            </div>
            <div class="instance-list-item type">
                {format!("{:#?}",&meta.servertype)}
            </div>
        </>
    }
}