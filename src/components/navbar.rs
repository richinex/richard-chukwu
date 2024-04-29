
// use yew::prelude::*;
// use crate::app::Route;
// use yew_router::prelude::*;

// #[function_component(NavBar)]
// pub fn nav_bar() -> Html {
//     let current_route: Option<Route> = use_route();

//     html! {
//         <nav class="bg-gray-900 text-white shadow-md sticky top-0 z-50">
//             <div class="container mx-auto px-4 py-2 flex justify-around">
//                 <Link<Route> to={Route::Home} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Home) { "active" } else { "" })}>
//                     <i class={"fas fa-home"}></i>{" Home"}
//                 </Link<Route>>
//                 <Link<Route> to={Route::Projects} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Projects) { "active" } else { "" })}>
//                     <i class={"fas fa-project-diagram"}></i>{" Projects"}
//                 </Link<Route>>
//                 <Link<Route> to={Route::Gallery} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Gallery) { "active" } else { "" })}>
//                     <i class={"fas fa-images"}></i>{" Gallery"}
//                 </Link<Route>>
//                 <Link<Route> to={Route::Articles} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Articles) { "active" } else { "" })}>
//                     <i class={"fas fa-newspaper"}></i>{" Articles"}
//                 </Link<Route>>
//                 <Link<Route> to={Route::Entertainment} classes={classes!("nav-link", "hover:text-gray-300", if current_route == Some(Route::Entertainment) { "active" } else { "" })}>
//                     <i class={"fas fa-question-circle"}></i>{" Trivia"}
//                 </Link<Route>>
//             </div>
//         </nav>
//     }
// }



use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let current_route: Option<Route> = use_route();
    let is_menu_open = use_state(|| false);
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open)
        })
    };

    html! {
        <nav class="bg-gray-900 text-white shadow-md sticky top-0 z-50">
            <div class="container mx-auto px-4 py-2 flex justify-around items-center">
                <button onclick={toggle_menu} class="menu-button">
                    <i class={"fas fa-bars"}></i>
                </button>
                <div class="nav-links">
                <Link<Route> to={Route::Home} classes={classes!("nav-link", if current_route == Some(Route::Home) { "active" } else { "hover:text-gray-300" })}>
                     <i class={"fas fa-home"}></i>{" Home"}
                </Link<Route>>
                <Link<Route> to={Route::Projects} classes={classes!("nav-link", if current_route == Some(Route::Projects) { "active" } else { "hover:text-gray-300" })}>
                    <i class={"fas fa-project-diagram"}></i>{" Projects"}
                </Link<Route>>
                <Link<Route> to={Route::Gallery} classes={classes!("nav-link", if current_route == Some(Route::Gallery) { "active" } else { "hover:text-gray-300" })}>
                    <i class={"fas fa-images"}></i>{" Gallery"}
                </Link<Route>>
                <Link<Route> to={Route::Articles} classes={classes!("nav-link", if current_route == Some(Route::Articles) { "active" } else { "hover:text-gray-300" })}>
                     <i class={"fas fa-newspaper"}></i>{" Articles"}
                </Link<Route>>
                <Link<Route> to={Route::Trivia} classes={classes!("nav-link", if current_route == Some(Route::Trivia) { "active" } else { "hover:text-gray-300" })}>
                    <i class={"fas fa-question-circle"}></i>{" Trivia"}
            </Link<Route>>
            </div>
                if *is_menu_open {
                    <div class="mobile-nav">
                    <Link<Route> to={Route::Home} classes={classes!("nav-link", if current_route == Some(Route::Home) { "active" } else { "hover:text-gray-300" })}>
                     <i class={"fas fa-home"}></i>{" Home"}
                    </Link<Route>>
                    <Link<Route> to={Route::Projects} classes={classes!("nav-link", if current_route == Some(Route::Projects) { "active" } else { "hover:text-gray-300" })}>
                        <i class={"fas fa-project-diagram"}></i>{" Projects"}
                    </Link<Route>>
                    <Link<Route> to={Route::Gallery} classes={classes!("nav-link", if current_route == Some(Route::Gallery) { "active" } else { "hover:text-gray-300" })}>
                        <i class={"fas fa-images"}></i>{" Gallery"}
                    </Link<Route>>
                    <Link<Route> to={Route::Articles} classes={classes!("nav-link", if current_route == Some(Route::Articles) { "active" } else { "hover:text-gray-300" })}>
                        <i class={"fas fa-newspaper"}></i>{" Articles"}
                    </Link<Route>>
                    <Link<Route> to={Route::Trivia} classes={classes!("nav-link", if current_route == Some(Route::Trivia) { "active" } else { "hover:text-gray-300" })}>
                        <i class={"fas fa-question-circle"}></i>{" Trivia"}
                </Link<Route>>
                    </div>
                }
            </div>
        </nav>
    }
}


