// components/navbar.rs
use yew::prelude::*;
use crate::app::Route;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    // Explicitly state the type expected from use_route()
    let current_route: Option<Route> = use_route();

    html! {
        <nav class="bg-gray-900 text-white shadow-md sticky top-0 z-50">
            <div class="container mx-auto px-4 py-2 flex justify-around">
                <Link<Route> to={Route::Home} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Home) { "active" } else { "" })}>
                    {"Home"}
                </Link<Route>>
                <Link<Route> to={Route::Projects} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Projects) { "active" } else { "" })}>
                    {"Projects"}
                </Link<Route>>
                <Link<Route> to={Route::Gallery} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Gallery) { "active" } else { "" })}>
                    {"Gallery"}
                </Link<Route>>
                <Link<Route> to={Route::Articles} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Articles) { "active" } else { "" })}>
                    {"Articles"}
                </Link<Route>>
                <Link<Route> to={Route::Secure} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Secure) { "active" } else { "" })}>
                    {"Secure"}
                </Link<Route>>
            </div>
        </nav>
    }
}
