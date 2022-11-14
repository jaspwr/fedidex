use yew::prelude::*;

#[function_component(TabelLabels)]
pub fn header() -> Html {
    html! {
        <div class="instance-list-item-container column-labels">
            <div class="instance-list-item favicon label"></div>
            <div class="instance-list-item name label">{"Domain"}</div>
            <div class="instance-list-item description label">{"Description"}</div>
            <div class="instance-list-item invite-only label">{"Open"}</div>
            <div class="instance-list-item users label">{"Users"}</div>
            <div class="instance-list-item type label">{"Platform"}</div>
        </div>
    }
}