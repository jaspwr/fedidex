use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <a href="https://github.com/jaspwr/fedidex"> { "Source" } </a>
        </div>
    }
}