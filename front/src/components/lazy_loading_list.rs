use web_sys::Window;
use web_sys::console;
use web_sys::window;
use yew::prelude::*;

use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use yew_hooks::use_measure;
use yew_hooks::use_scroll;
use yew_hooks::use_size;

use crate::components::TabelLabels;
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

#[function_component(LazyLoadingList)]
pub fn lazy_loading_list(props: &Props) -> Html {
    let Props { query } = props.clone();
    let node = use_node_ref();
    let node_inner = use_node_ref();
    let scroll = use_scroll(node.clone());
    let pending_scroll_up = use_state_eq(|| false);
    let furthest_scroll_at_last_load = use_state_eq(|| 0);
    let page = use_state_eq(|| 0);
    let pre_query_state = use_state_eq(|| "".to_string());
    let list_size = use_size(node_inner.clone());
    let container_size = use_size(node.clone());
    if (*pending_scroll_up).clone() {
        if scroll.1 < 100 { pending_scroll_up.set(false); } 
        else { return html! { <> <div class="instance-list" id="instance-list" ref={node}><div class="instance-list-item-container" ref={node_inner}></div></div> </> }; }
    }
    if query != *pre_query_state {
        page.set(0);
        furthest_scroll_at_last_load.set(0);
        pre_query_state.set(query.clone());
        pending_scroll_up.set(true);
    }
    if (list_size.1 as i32 - scroll.1) < 800 && *furthest_scroll_at_last_load + 1000 < scroll.1 {
        page.set(*page + 1);
        furthest_scroll_at_last_load.set(scroll.1);
    }
    
    html! {
        <>
        <div class="instance-list" id="instance-list" ref={node}>
            {
                if container_size.0 < 500 {
                    html! {
                        <div class="table-labels-mobile">
                            <TabelLabels/> 
                        </div>
                    }
                } else {
                    html! { <></> }
                }
            }
            <div class="instance-list-item-container" ref={node_inner}>
                < InstanceListWrapper query={ (*pre_query_state).clone() } page={0}/>
                {
                    for (0..*page).map(|i| html!{< InstanceListWrapper query={ (*pre_query_state).clone() } page={i + 1}/>})
                }
            </div>
        </div>
        </>
    }
}