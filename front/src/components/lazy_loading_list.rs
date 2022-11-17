use yew::prelude::*;

use yew_hooks::use_scroll;
use yew_hooks::use_size;

use crate::components::InstanceListWrapper;
use crate::components::TabelLabels;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub query: String,
}

#[function_component(LazyLoadingList)]
pub fn lazy_loading_list(props: &Props) -> Html {
    let Props { query } = props.clone();
    let node = use_node_ref();
    let node_inner = use_node_ref();
    let scroll = use_scroll(node.clone());
    let pending_scroll_up = use_state_eq(|| false);
    let list_height_at_last_load = use_state_eq(|| 0);
    let page = use_state_eq(|| 0);
    let pre_query_state = use_state_eq(|| "".to_string());
    let list_size = use_size(node_inner.clone());
    let container_size = use_size(node.clone());
    let pre_small = use_state_eq(|| false);
    let small = container_size.0 < 500;
    if (*pending_scroll_up).clone() {
        if scroll.1 == 0 && list_size.1 < 20 { pending_scroll_up.set(false); pre_small.set(small); } 
            else { return html! { <> <div class="instance-list" id="instance-list" ref={node}><div class="instance-list-item-container" ref={node_inner}></div></div> </> }; }
    }
    if query != *pre_query_state || *pre_small != small {
        page.set(0);
        list_height_at_last_load.set(0);
        pre_query_state.set(query.clone());
        pending_scroll_up.set(true);
    }
    if (list_size.1 as i32 - (scroll.1 + container_size.1 as i32)) < 50 && list_size.1 as i32 != *list_height_at_last_load {
        page.set(*page + 1);
        list_height_at_last_load.set(list_size.1 as i32);
    }
    html! {
        <>
        <div class="instance-list" id="instance-list" ref={node}>
            <div class="instance-list-item-container" ref={node_inner}>
            // //TODO: Fix crashing with this
                // {
                //     if *pre_small {
                //         html! { <TabelLabels/> }
                //     } else {
                //         html! { <></> }
                //     }
                // }
                {
                    for (-1..*page).map(|i| html!{< InstanceListWrapper query={ (*pre_query_state).clone() } page={i + 1}/>})
                }
            </div>
        </div>
        </>
    }
}