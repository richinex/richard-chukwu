use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));

    html! {
        <div class="secure-page min-h-screen flex flex-col items-center justify-center p-4">
            // Removed specific text color and background classes to use the global style
            <h1 class="text-2xl font-bold mb-4">{ "Secure Area" }</h1>
            <button onclick={onclick_callback} class="button px-6 py-3 font-semibold rounded-md shadow transition-colors duration-300 focus:outline-none focus:ring-2 focus:ring-opacity-50">
                { "Go Home" }
            </button>
        </div>
    }
}
