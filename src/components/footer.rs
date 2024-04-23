// components/footer.rs
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-gray-800 text-white text-center p-4">
            <div class="container mx-auto">
                <p class="text-sm">{ "© 2024 Richard. All rights reserved." }</p>
            </div>
        </footer>
    }
}
