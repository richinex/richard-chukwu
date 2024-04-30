//home.rs
use yew::prelude::*;


use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().expect("Navigator hook used outside <BrowserRouter/> context");

    // Event callback to navigate to the projects page
    let navigate_to_projects = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Projects); // Assuming `Projects` is a valid route enum or path
        })
    };

    let faux_code = html! {
        <>
            <span class="attribute">{"#[macro_export]"}</span><br/>
            <span class="keyword">{"macro_rules!"}</span>{" "}
            <span class="function">{"greeting"}</span>{" {"}<br/>
            <span>{"    ("}</span><span class="variable">{"_who:ident"}</span><span>{") => {"}</span><br/>
            <span>{"        "}<span class="string">{"println!(\"Hello, I'm "}<span class="highlight-name">{"Richard Chukwu"}</span>{", a passionate "}<span class="highlight-role">{"DevOps engineer"}</span>{" and "}<span class="highlight-role">{"Rustacean"}</span>{"!\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"I'm currently building my personal website with "}<span class="highlight-language">{"Rust and Webassembly"}</span> {" (WASM).\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"On weekends I do a bit of resting and working on my personal projects"}{".\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"At work, besides the usual DevOps, I code mostly in Python and Java"}{".\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"These days I tend to like more of the low level stuff hence the reason I find Rust appealing"}{".\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\" "}<span class="highlight-language">{"Rust "}</span>{"is "}<span class="highlight-tech">{"fast, safe and has this type and ownership system that gives it fearless concurrency"}</span>{".\");"}</span></span><br/>
            <span>{"    };"}</span><br/>
            <span>{"}"}</span><br/>
        </>
    };

    html! {
        <>
            <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
            <h1 class="text-2xl md:text-5xl font-bold transition duration-500 ease-in-out hover:text-purple-300">{ "About Me" }</h1>
            </header>
            // <main class="flex flex-col md:flex-row-reverse items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
            <main class="flex flex-col items-center justify-center p-4 w-full text-center md:text-left">
                <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg transition duration-700 ease-in-out hover:rotate-12 hover:bg-purple-600">
                    <img src="/richard-chukwu/images/portfolio_image_01.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64 transition-transform duration-500 ease-in-out hover:scale-125" alt="Profile Image" />
                </div>
                <div class="code-snippet space-y-6 p-6 md:col-span-2 text-white">
                    <pre class="large-font"><code class="language-rust">{faux_code}</code></pre>
                    <div class="project-link mt-8">
                        <a onclick={navigate_to_projects} class="text-blue-500 hover:text-blue-700 cursor-pointer">
                        { "Click here to see some of my projects" }
                        </a>
                    </div>
                </div>
            </main>
        </>
    }


}





// use yew::{function_component, html, use_effect_with, use_node_ref, use_state, Callback, Html};
// use web_sys::HtmlElement;
// use gloo_timers::callback::Interval;
// use std::rc::Rc;
// use std::cell::RefCell;

// #[function_component(Home)]
// pub fn home() -> Html {
//     let hover_state = use_state(|| false);
//     let code_node_ref = use_node_ref();
//     let text = Rc::new("fn main() {\n    println!(\"Hello, world!\");\n}".to_string());
//     let pos = Rc::new(RefCell::new(0));

//     // Setup the typing animation to start immediately when the component mounts
//     // Clone `code_node_ref` here for use in the effect
//     let code_node_ref_clone = code_node_ref.clone();
//     use_effect_with(
//         (),
//         move |_| {
//             let code_node_ref = code_node_ref.clone();
//             let text = text.clone();
//             let pos = pos.clone();
//             Interval::new(100, move || {
//                 let mut pos_mut = pos.borrow_mut();
//                 if *pos_mut < text.len() {
//                     let ch = &text[*pos_mut..=*pos_mut];
//                     if let Some(element) = code_node_ref.cast::<HtmlElement>() {
//                         element.set_inner_text(&format!("{}{}", element.inner_text(), ch));
//                     }
//                     *pos_mut += 1;
//                 }
//             }).forget();
//             || ()
//         }
//     );

//     // Hover handlers for the image
//     let handle_mouseover = {
//         let hover_state = hover_state.clone();
//         Callback::from(move |_| hover_state.set(true))
//     };

//     let handle_mouseout = {
//         let hover_state = hover_state.clone();
//         Callback::from(move |_| hover_state.set(false))
//     };

//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
//             <div class="bg-opacity-80 bg-rich-blue rounded-full shadow-lg p-4">
//             <img src="images/richie_davinci2.png"
//                  class="rounded-full border-4 border-secondary shadow-xl"
//                  alt="Profile Image"
//                  style="width: 256px; height: 256px;"
//                  onmouseover={handle_mouseover}
//                  onmouseout={handle_mouseout} />
//         </div>

//                 <div class="space-y-6 p-4 md:col-span-2 text-white">
//                     <h2 class="text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "Explore more about Rust and other technologies on my blog." }</p>
//                 </div>
//                 <div class="code-snippet col-span-3">
//                     <pre><code class="language-rust" ref={code_node_ref_clone}>{ " " }</code></pre>
//                 </div>
//             </main>
//         </>
//     }
// }



// use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <div id="home-container" class="book-layout">
//             <h1>{ "Welcome to My Personal Blog" }</h1>
//             <div class="content">
//                 <p class="text">
//                     { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
//                 </p>
//                 <img class="top-right-image" src="images/richie_davinci2.png" alt="Placeholder Image"/>
//             </div>
//         </div>
//     }
// }

// home.rs

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <div id="home-container" class="book-layout">
//             <h1>{ "Welcome to My Personal Blog" }</h1>
//             <div class="content">
//                 <div class="text-column">
//                     <p>
//                         { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
//                     </p>
//                     <p>
//                         { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
//                     </p>
//                 </div>
//                 <div class="text-column">
//                     <p>
//                         { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
//                     </p>
//                     <p>
//                         { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
//                     </p>
//                 </div>
//                 <img class="top-right-image" src="https://via.placeholder.com/150" alt="Placeholder Image"/>
//             </div>
//         </div>
//     }
// }

// #[function_component(Home)]
// pub fn home() -> Html {
//     // Use `use_effect` to run some code after the component renders
//     use_effect(|| {
//         // Code to run after the component renders
//         web_sys::console::log_1(&"Home component rendered".into());

//         // Optional cleanup code
//         || {
//             web_sys::console::log_1(&"Cleaning up Home component".into());
//         }
//     });

//     html! {
//         <div>
//             <h1>{ "Welcome to My Personal Blog" }</h1>
//             <p>{ "This is the home page." }</p>
//         </div>
//     }
// }
