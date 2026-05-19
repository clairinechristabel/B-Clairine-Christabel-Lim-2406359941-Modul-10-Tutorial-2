use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-black flex w-screen h-screen items-center justify-center font-mono">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <h1 class="text-green-500 text-4xl mb-8 tracking-widest drop-shadow-[0_0_10px_rgba(34,197,94,0.8)]">{"[ GALACTIC TERMINAL ]"}</h1>
                <form class="m-4 flex flex-col items-center shadow-[0_0_20px_rgba(34,197,94,0.3)] bg-gray-900 p-10 rounded-xl border border-green-700">
                    <p class="text-green-400 mb-4 text-sm animate-pulse">{"PLEASE ENTER YOUR DESIGNATION"}</p>
                    <input {oninput} class="rounded p-4 text-green-400 border border-green-600 bg-black outline-none mb-6 w-full text-center focus:shadow-[0_0_15px_rgba(34,197,94,0.6)]" placeholder="Commander_Name" />
                    <Link<Route> to={Route::Chat}> 
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded bg-green-900 hover:bg-green-700 text-green-200 font-bold p-4 uppercase border border-green-500 w-full transition duration-300" >
                            {"INITIALIZE UPLINK"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}
