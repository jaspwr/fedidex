use web_sys::Window;
use web_sys::console;
use web_sys::window;
use yew::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew_hooks::use_scroll;
use yew_hooks::use_update;

use crate::components::InstanceListWrapper;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub query: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn entry (i: i32, state: i32, query: &String) -> Html {
    if state > 1500 * i { html! { < InstanceListWrapper query={ (*query).clone() } page={i}/> } } else { html! { <></> } }
}

#[function_component(LazyLoadingList)]
pub fn lazy_loading_list(props: &Props) -> Html {
    let Props { query } = props.clone();
    let node = use_node_ref();
    let state = use_scroll(node.clone());
    let pending_scroll_up = use_state_eq(|| false);
    let furthest_scroll = use_state_eq(|| 0);
    let pre_query_state = use_state_eq(|| "".to_string());
    if (*pending_scroll_up).clone() {
        if state.1 < 100 { pending_scroll_up.set(false); } else { return html! {<> <div class="instance-list" id="instance-list" ref={node}></div> </> }; }
    }
    if query != *pre_query_state {
        pending_scroll_up.set(true);
        furthest_scroll.set(0);
        pre_query_state.set(query.clone());
    }
    if state.1 > (*furthest_scroll).clone() {
        furthest_scroll.set(state.1);
    }
    html! {
        <>
        <div class="instance-list" id="instance-list" ref={node}>
            <div class="instance-list-item-container">
                < InstanceListWrapper query={ (*pre_query_state).clone() } page={0}/>
                { for (1..100).map(|i| entry(i, (*furthest_scroll).clone(), &(*pre_query_state))) }
            </div>
        </div>
        </>
    }
}