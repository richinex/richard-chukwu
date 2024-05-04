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
            <span>{"        "}<span class="string">{"println!(\"At work, besides the usual DevOps, I code mostly in Rust, Python and Java"}{".\");"}</span></span><br/>
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
                <div class="code-block">
                    <div class="window-controls">
                        <span class="dot" style="background-color: #ff5f56;"></span>
                        <span class="dot" style="background-color: #ffbd2e;"></span>
                        <span class="dot" style="background-color: #27c93f;"></span>
                    </div>
                    <pre class="large-font"><code class="language-rust">{faux_code}</code></pre>
                </div>
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



