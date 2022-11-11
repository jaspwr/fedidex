use yew::prelude::*;

use crate::instance_list::InstanceList;
use gloo_net::http::Request;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub query: String
}

#[function_component(InstanceListWrapper)]
pub fn instance_list_wrapper(props: & Props) -> Html {
    let search = props.query.clone();
    drop(props);
    let instance_list = use_state(|| vec![]);
    {
        let instance_list = instance_list.clone();
        use_effect_with_deps(move |_| {
            let instance_list = instance_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get(&format!("http://127.0.0.1:8000/search/t{}", search))
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                let spl = resp.split('\n');
                let _list = spl.map(|x| x.to_string()).collect::<Vec<String>>();
                instance_list.set(_list);
            });
            || ()
        }, ());
    }
    html! {
        < InstanceList items={(*instance_list).clone()}/>
    }
}