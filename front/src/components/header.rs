use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="header">
            <div class="heading">
                {"fedidex [alpha]"}
            </div>
            <div class="subheading">
                {"Directory of instances for Mastodon, Pleroma, ect. found by crawlers."}
            </div>
        </div>

    }
}