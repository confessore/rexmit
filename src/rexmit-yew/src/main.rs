pub mod components;

use yew::prelude::*;
use crate::components::footer::Footer;

#[function_component]
fn App() -> Html {

    html! {
        <>
            <div class="flex flex-col">
                <div >
                    <a href="https://discord.com/api/oauth2/authorize?client_id=1021189711366213672&permissions=0&scope=bot"><button>{"add to your server"}</button></a>
                </div>
                <Footer />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}