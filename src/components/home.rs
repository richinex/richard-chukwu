//home.rs
// home.rs
use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-rich-blue text-center p-4">
//                 // The text color will be inherited from the body style defined in SCSS
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="flex flex-col items-center p-4">
//                 <img src="images/richie_davinci2.png" class="mb-8 rounded-full w-48 h-auto" alt="Profile Image" />
//                 <div class="text-center space-y-6 max-w-2xl">
//                     // The text color will be inherited from the body style defined in SCSS
//                     <h2 class="text-2xl md:text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "I'm probably the most passionate DevOps engineer you will ever get to work with. If you have a great project that needs some amazing skills, I'm your guy." }</p>
//                     <p class="leading-relaxed">{ "I'm currently building my personal website with Rust and WebAssembly (WASM). My choice framework is Yew." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                     // More paragraphs or content...
//                 </div>
//             </main>
//         </>
//     }
// }

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="flex flex-col md:flex-row items-center justify-center p-4 text-center md:text-left w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black">
//                 <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg">
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64" alt="Profile Image" />
//                 </div>
//                 <div class="space-y-6 p-4 max-w-2xl text-white">
//                     <h2 class="text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "I'm probably the most passionate DevOps engineer you will ever get to work with. If you have a great project that needs some amazing skills, I'm your guy." }</p>
//                     <p class="leading-relaxed">{ "I'm currently building my personal website with Rust and WebAssembly (WASM). My choice framework is Yew." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                 </div>
//             </main>
//         </>
//     }
// }

// use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="flex flex-col md:flex-row items-center justify-center p-4 text-center md:text-left w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black">
//                 <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg">
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64" alt="Profile Image" />
//                 </div>
//                 // Increased margin on the left for the text container when in md:flex-row layout
//                 <div class="space-y-6 p-4 max-w-2xl text-white md:ml-10">
//                     <h2 class="text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "I'm probably the most passionate DevOps engineer you will ever get to work with. If you have a great project that needs some amazing skills, I'm your guy." }</p>
//                     <p class="leading-relaxed">{ "I'm currently building my personal website with Rust and WebAssembly (WASM). My choice framework is Yew." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                 </div>
//             </main>
//         </>
//     }
// }

// transition but no space
// use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="flex flex-col md:flex-row items-center justify-center p-4 text-center md:text-left w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black">
//                 <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg transition-all duration-500 ease-in-out hover:bg-opacity-100">
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64 transition-transform duration-300 ease-in-out hover:scale-110" alt="Profile Image" />
//                 </div>
//                 <div class="space-y-6 p-4 max-w-2xl text-white transition-all duration-500 ease-in-out hover:text-secondary">
//                     <h2 class="text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "I'm probably the most passionate DevOps engineer you will ever get to work with. If you have a great project that needs some amazing skills, I'm your guy." }</p>
//                     <p class="leading-relaxed">{ "I'm currently building my personal website with Rust and WebAssembly (WASM). My choice framework is Yew." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                     <p class="leading-relaxed">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="leading-relaxed">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                 </div>
//             </main>
//         </>
//     }
// }

// final version

// use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold transition duration-500 ease-in-out hover:text-purple-300">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="flex flex-col md:flex-row items-center justify-center p-4 text-center md:text-left w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black">
//                 <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg transition duration-700 ease-in-out hover:rotate-12 hover:bg-purple-600">
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64 transition-transform duration-500 ease-in-out hover:scale-125" alt="Profile Image" />
//                 </div>
//                 <div class="space-y-6 p-4 max-w-2xl text-white transition-colors duration-500 ease-in-out hover:text-light-blue-200">
//                     <h2 class="text-3xl font-bold transition-all duration-500 ease-in-out hover:underline hover:text-secondary">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     // Ensuring all paragraph tags have unique classes for targeted styling
//                     <p class="p-text leading-relaxed transition-all delay-150 duration-500 ease-in-out hover:text-secondary">{ "I'm probably the most passionate DevOps engineer you will ever get to work with. If you have a great project that needs some amazing skills, I'm your guy." }</p>
//                     <p class="p-text leading-relaxed transition-all delay-300 duration-500 ease-in-out hover:text-secondary">{ "I'm currently building my personal website with Rust and WebAssembly (WASM). My choice framework is Yew." }</p>
//                     <p class="p-text leading-relaxed transition-all delay-450 duration-500 ease-in-out hover:text-secondary">{ "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }</p>
//                     <p class="p-text leading-relaxed transition-all delay-600 duration-500 ease-in-out hover:text-secondary">{ "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }</p>
//                 </div>
//             </main>
//         </>
//     }
// }

// code version
// use yew::prelude::*;

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
//                 <div class="bg-opacity-80 bg-rich-blue rounded-full shadow-lg p-4">
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl animate-scale" alt="Profile Image" style="width: 256px; height: 256px;" />
//                 </div>
//                 <div class="space-y-6 p-4 md:col-span-2 text-white">
//                     <h2 class="text-3xl font-bold">{ "I'm Paparichie. I'm a passionate DevOps engineer, Rustacean, and Java dev." }</h2>
//                     <p class="leading-relaxed">{ "Explore more about Rust and other technologies on my blog." }</p>
//                 </div>
//                 <div class="code-snippet col-span-3">
//                     <pre><code class="language-rust">{ "fn main() {\n    println!(\"Hello, world!\");\n}" }</code></pre>
//                 </div>
//             </main>
//         </>
//     }
// }

// use yew::{function_component, html, use_effect_with, use_state, use_node_ref, Html, Callback};
// use web_sys::{HtmlElement, console};
// use gloo_timers::callback::Timeout;
// use std::rc::Rc;
// use std::cell::RefCell;

// #[function_component(Home)]
// pub fn home() -> Html {
//     let hover_state = use_state(|| false);
//     let code_node_ref = use_node_ref();
//     let pos = Rc::new(RefCell::new(0));  // Mutable position counter

//     let hover_state_mouseover = hover_state.clone();
//     let hover_state_mouseout = hover_state.clone();
//     let pos_effect = pos.clone();

//     use_effect_with(
//         hover_state,
//         move |hover| {
//             let pos = pos_effect.clone();
//             console::log_1(&"Effect triggered".into());
//             if **hover {
//                 console::log_1(&"Hover is true".into());
//                 if let Some(element) = code_node_ref.cast::<HtmlElement>() {
//                     element.set_inner_text(""); // Clear previous text if any
//                     console::log_1(&"Text cleared".into());
//                     *pos.borrow_mut() = 0; // Reset the position
//                     let text = "fn main() {\n    println!(\"Hello, world!\");\n}";
//                     let timeout = Timeout::new(100, move || {
//                         let mut pos_mut = pos.borrow_mut();
//                         if *pos_mut < text.len() {
//                             let ch = &text[*pos_mut..=*pos_mut];
//                             element.set_inner_text(&format!("{}{}", element.inner_text(), ch));
//                             console::log_1(&format!("Writing character: {}", ch).into());
//                             *pos_mut += 1;
//                         }
//                     });
//                     timeout.forget(); // Prevent the timer from being destroyed
//                 }
//             } else {
//                 console::log_1(&"Hover is false".into());
//             }
//             || ()
//         }
//     );

// use yew::{function_component, html, use_effect_with, use_state, use_node_ref, Html, Callback};
// use web_sys::{HtmlElement, console};
// use std::rc::Rc;
// use std::cell::RefCell;

// #[function_component(Home)]
// pub fn home() -> Html {
//     let hover_state = use_state(|| false);
//     let code_node_ref = use_node_ref();
//     let code_node_ref_clone = code_node_ref.clone();
//     let pos = Rc::new(RefCell::new(0)); // Position for the typing effect

//     // Ensure hover_state is captured properly for effect triggers
//     let hover_effect = hover_state.clone();
//     let code_ref_effect = code_node_ref.clone();
//     let hover_state_mouseover = hover_state.clone();
//     let hover_state_mouseout = hover_state.clone();

//     use_effect_with(
//         hover_effect,
//         move |hover| {
//             if **hover {
//                 if let Some(element) = code_ref_effect.cast::<HtmlElement>() {
//                     element.set_inner_text(""); // Clear previous text if any
//                     console::log_1(&"Text cleared".into());
//                     let text = "fn main() {\n    println!(\"Hello, world!\");\n}".to_string();
//                     *pos.borrow_mut() = 0; // Reset the position

//                     let pos_clone = pos.clone();
//                     let element_clone = element.clone();
//                     gloo_timers::callback::Interval::new(100, move || {
//                         let mut pos_mut = pos_clone.borrow_mut();
//                         if *pos_mut < text.len() {
//                             let ch = &text[*pos_mut..=*pos_mut];
//                             element_clone.set_inner_text(&format!("{}{}", element_clone.inner_text(), ch));
//                             console::log_1(&format!("Writing character: {}", ch).into());
//                             *pos_mut += 1;
//                         }
//                     }).forget(); // Keep the interval running
//                 }
//             }
//             || ()
//         }
//     );

//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
//                 <div class="bg-opacity-80 bg-rich-blue rounded-full shadow-lg p-4"
//                      onmouseover={Callback::from(move |_| hover_state_mouseover.set(true))}
//                      onmouseout={Callback::from(move |_| hover_state_mouseout.set(false))}>
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl" alt="Profile Image" style="width: 256px; height: 256px;" />
//                 </div>
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

// working version
// use yew::{function_component, html, use_effect_with, use_state, Callback, Html};
// use web_sys::HtmlElement;
// use gloo_timers::callback::{Timeout, Interval};
// use std::rc::Rc;
// use std::cell::RefCell;

// #[function_component(Home)]
// pub fn home() -> Html {
//     let hover_state = use_state(|| false);
//     let hover_state_clone = hover_state.clone();
//     let code_node_ref = use_node_ref();
//     let code_node_ref_clone = code_node_ref.clone();
//     let pos = Rc::new(RefCell::new(0));
//     let text = Rc::new("fn main() {\n    println!(\"Hello, world!\");\n}".to_string());

//     let handle_mouseover = {
//         let hover_state = hover_state.clone();
//         Callback::from(move |_| hover_state.set(true))
//     };

//     let handle_mouseout = {
//         Callback::from(move |_| {
//             let hover_state = hover_state_clone.clone();
//             // Delay mouse out effect to allow for completion of the typing animation
//             Timeout::new(300, move || {
//                 hover_state.set(false);
//             }).forget();
//         })
//     };

//     // Effect to handle typing animation
//     use_effect_with(
//         hover_state,
//         move |hover| {
//             if **hover {
//                 if let Some(element) = code_node_ref.cast::<HtmlElement>() {
//                     element.set_inner_text(""); // Clear text initially
//                     let text = text.clone();
//                     let pos = pos.clone();
//                     let element = element.clone();

//                     Interval::new(300, move || {
//                         let mut pos_mut = pos.borrow_mut();
//                         if *pos_mut < text.len() {
//                             let ch = &text[*pos_mut..=*pos_mut];
//                             element.set_inner_text(&format!("{}{}", element.inner_text(), ch));
//                             *pos_mut += 1;
//                         }
//                     }).forget();
//                 }
//             }
//             || ()
//         }
//     );

//     html! {
//         <>
//             <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
//                 <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
//             </header>
//             <main class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
//                 <div class="bg-opacity-80 bg-rich-blue rounded-full shadow-lg p-4"
//                      onmouseover={handle_mouseover}
//                      onmouseout={handle_mouseout}>
//                     <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl" alt="Profile Image" style="width: 256px; height: 256px;" />
//                 </div>
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
            <span>{"        "}<span class="string">{"println!(\"Hello, I'm "}<span class="highlight-name">{"Paparichie"}</span>{", a passionate "}<span class="highlight-role">{"DevOps engineer"}</span>{" and "}<span class="highlight-role">{"Rustacean"}</span>{"!\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"I'm currently building my personal website with "}<span class="highlight-language">{"Rust and Webassembly"}</span> {" (WASM).\");"}</span></span><br/>
            <span>{"        "}<span class="string">{"println!(\"My choice framework for the frontend is "}<span class="highlight-tech">{"Yew"}</span>{".\");"}</span></span><br/>
            <span>{"    };"}</span><br/>
            <span>{"}"}</span><br/>
        </>
    };

    html! {
        <>
            <header class="bg-gradient-to-r from-rich-blue via-purple-500 to-rich-blue text-white text-center p-4">
                <h1 class="text-2xl md:text-5xl font-bold">{ "Welcome to My Personal Blog" }</h1>
            </header>
            <main class="flex flex-col md:flex-row-reverse items-center justify-center p-4 w-full bg-gradient-to-br from-gray-800 via-gray-900 to-black text-center md:text-left">
                <div class="m-4 p-4 bg-opacity-80 bg-rich-blue rounded-full shadow-lg transition duration-700 ease-in-out hover:rotate-12 hover:bg-purple-600">
                    <img src="images/richie_davinci2.png" class="rounded-full border-4 border-secondary shadow-xl w-64 h-64 transition-transform duration-500 ease-in-out hover:scale-125" alt="Profile Image" />
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
