

\`\`\`rust
#[macro_export]
macro_rules! greeting {
    (_who:ident) => {
        println!("Hello, I'm {}, a passionate DevOps engineer and Rustacean!", _who);
        println!("I'm currently building my personal website with Rust and Webassembly (WASM).");
        println!("My choice framework for the frontend is Yew.");
    };
}
\`\`\`

