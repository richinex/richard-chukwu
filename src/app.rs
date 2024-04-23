use yew::functional::*;
use yew::prelude::*;
use crate::components::{footer, gallery, home, projects, secure, navbar, articles};
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/gallery")]
    Gallery,
    #[at("/secure")]
    Secure,
    #[at("/article")]
    Articles,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex flex-col min-h-screen">
                <navbar::NavBar />
                <main class="flex-grow">
                <Switch<Route> render={switch} />
                </main>
                <footer::Footer />
            </div>
        </BrowserRouter>
    }
}

pub fn switch(routes: Route) -> Html {
    web_sys::console::log_1(&format!("Routing to {:?}", routes).into());
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Projects => html! { <projects::Projects /> },
        Route::Gallery => html! { <gallery::Gallery /> },
        Route::Secure => html! { <secure::Secure /> },
        Route::Articles => html! { <articles::Articles /> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}
