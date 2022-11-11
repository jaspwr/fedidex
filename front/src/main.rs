use yew::prelude::*;

use crate::InstanceList;


pub mod components;
use crate::components::*;


pub enum Msg {
    NewSearchTerm(String),
}

#[derive(Debug, Default)]
pub struct App {
    search_term: String,
    instance_list: [String; 3]
}



impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            search_term: "".to_string(), 
            instance_list: ["".to_string(), "".to_string(), "".to_string()]
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
                < InstanceListWrapper query={self.search_term.clone()}/>
            </div>
            < Footer />
            </>
        }
    }
}

fn main() -> () {
    yew::start_app::<App>();
}