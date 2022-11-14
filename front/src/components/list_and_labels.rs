use yew::prelude::*;
use yew_hooks::use_size;

use crate::lazy_loading_list::LazyLoadingList;
use crate::tabel_labels::TabelLabels;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub query: String,
}

#[function_component(ListAndLabels)]
pub fn list_and_labels(props: &Props) -> Html {
    let list_node = use_node_ref();
    let list_size = use_size(list_node.clone());
    html! {
        <>
            {
                if list_size.0 >= 500 {
                    html! { <TabelLabels/> }
                } else {
                    html! { <></> }
                }
            }
            < LazyLoadingList query={ props.query.clone()} ref={list_node} />
        </>
    }
}