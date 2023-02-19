# rust-lab
A lab where I tinker with the Rust language.

## References

1. [YouTube | Let's Build a RUST WebAssembly Frontend App with Yew](https://www.youtube.com/watch?v=lJllt5X6ELg)

2. [Article | The current state of Rust web frameworks](https://blog.logrocket.com/current-state-rust-web-frameworks/)

## Log

February 19, 2023

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). You need to add `$HOME/.cargo/bin` to your `$PATH` variable in `~/.zshrc` if you're using ZSH.

2. `cargo install trunk`

3. `rustup target add wasm32-unknown-unknown`

4. `cargo new yew-video-streaming`