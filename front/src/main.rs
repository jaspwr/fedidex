#![recursion_limit="1000"]
use yew::prelude::*;
use yew_hooks::use_scroll;

pub mod components;
use crate::components::*;


pub enum Msg {
    NewSearchTerm(String),
}

#[derive(Debug, Default)]
pub struct App {
    search_term: String,
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            search_term: "".to_string(), 
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewSearchTerm(_term) => {
                self.search_term = _term;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::NewSearchTerm);
        html! {
            <>
            <div class="content">
                < Header />
                < SearchBox value={self.search_term.clone()} {on_change}/>
                <div class="instance-list-item-container column-labels">
                    <div class="instance-list-item favicon label"></div>
                    <div class="instance-list-item name label">{"Domain"}</div>
                    <div class="instance-list-item description label">{"Description"}</div>
                    <div class="instance-list-item invite-only label">{"Open"}</div>
                    <div class="instance-list-item users label">{"Users"}</div>
                    <div class="instance-list-item type label">{"Platform"}</div>
                </div>
                < LazyLoadingList query={ self.search_term.clone() } />
            </div>
            < Footer />
            </>
        }
    }
}

fn main() -> () {
    yew::start_app::<App>();
    //yew::Renderer::<App>::new().render();
}