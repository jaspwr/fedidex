use yew::prelude::*;

use crate::InstanceList;

mod components;
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
            instance_list: ["{\"address\":\"https://metalhead.club\",\"servertype\":\"Mastodon\",\"name\":\"metalhead.club\",\"description\":\"Metgy.\",\"users\":0,\"favicon\":\"https://metalhead.club/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668049881}".to_string(),
            "{\"address\":\"https://mastodon.social\",\"servertype\":\"Mastodon\",\"name\":\"mastodon.social\",\"description\":\"The original server operated by the Mastodon gGmbH non-profit\",\"users\":0,\"favicon\":\"https://mastodon.social/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668058877}".to_string(),
                            "{\"address\":\"https://metalhead.club\",\"servertype\":\"Mastodon\",\"name\":\"metalhead.club\",\"description\":\"Metalhead.club is a Mastodon instance hosted in Germany and powered by 100% green energy.\",\"users\":0,\"favicon\":\"https://metalhead.club/favicon.ico\",\"invite_only\":true,\"last_indexed\":1668049881}".to_string()
        ]}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewSearchTerm(_term) => { 
                self.search_term = _term;
                self.instance_list = [
                        "{\"address\":\"https://metalhead.club\",\"servertype\":\"Mastodon\",\"name\":\"metalsssssshead.club\",\"description\":\"Metalhssssssssssssssssssssssead.club is a Mastodon instance hosted in Germany and powered by 100% green energy.\",\"users\":0,\"favicon\":\"https://metalhead.club/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668049881}".to_string(),
                        "{\"address\":\"https://metalhead.club\",\"servertype\":\"Mastodon\",\"name\":\"metalhead.club\",\"description\":\"Metalhead.club is a Mastodon instance hosted in Germany and powered by 100% green energy.\",\"users\":0,\"favicon\":\"https://metalhead.club/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668049881}".to_string(),
                        "".to_string()
                    ];
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
                < InstanceList items={self.instance_list.clone()}/>
            </div>
            < Footer />
            </>
        }
    }
}

fn main() -> () {
    yew::start_app::<App>();
}