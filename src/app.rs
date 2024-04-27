use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{footer, gallery, home, projects, navbar, articles, trivia};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/gallery")]
    Gallery,
    #[at("/entertainment")]
    Entertainment,
    #[at("/articles")]
    Articles,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div class="flex flex-col min-h-screen">
                <navbar::NavBar />
                <main class="flex-grow">
                    <Switch<Route> render={switch} />
                </main>
                <footer::Footer />
            </div>
        </HashRouter>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <home::Home /> },
        Route::Projects => html! { <projects::Projects /> },
        Route::Gallery => html! { <gallery::Gallery /> },
        Route::Entertainment => html! { <trivia::Entertainment /> },
        Route::Articles => html! { <articles::Articles /> },
        Route::NotFound => html! { <h1>{"404 Not Found"}</h1> },
    }
}

