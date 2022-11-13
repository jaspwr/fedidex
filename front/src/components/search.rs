use yew::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    let window = web_sys::window().unwrap();
    let inst = window.document().unwrap().get_element_by_id("instance-list");
    if inst.is_some() {
        inst.unwrap().set_scroll_top(0);
    }
    target.value()
}

#[function_component(SearchBox)]
pub fn search_box(props: &Props) -> Html {
    let Props { value, on_change } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <>
            <span id="search-label">
                {"Search:"}
            </span>
            <input type="text" class="search" {value} {oninput}/>
        </>
    }
}