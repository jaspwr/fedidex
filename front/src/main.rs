use yew::prelude::*;

mod components;
use crate::components::*;

#[function_component(App)]
fn app() -> Html {
    let i = Instance { 
        name: String::from("fuck"),
        url: String::from("you"),
        users: 0,
        invite_only: true,
    };
    html! {
        <>
        <div class="content">
            < Header />
            < SearchBox value="meow"/>
            < InstanceList items={[i.clone(), i.clone(), i.clone()]}/>
        </div>
        < Footer />
        </>
    }
}

fn main() -> () {
    yew::start_app::<App>();
}