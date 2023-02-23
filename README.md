# rust-lab
A lab where I tinker with the Rust language.

## References

1. [YouTube | Let's Build a RUST WebAssembly Frontend App with Yew](https://www.youtube.com/watch?v=lJllt5X6ELg)

2. [Article | The current state of Rust web frameworks](https://blog.logrocket.com/current-state-rust-web-frameworks/)

3. [Docs | Yew Tutorial](https://yew.rs/docs/tutorial)

4. [Book | The Rust Programming Language](https://doc.rust-lang.org/book/ch00-00-introduction.html)

## Log

February 20, 2023

1. Add `yew` as a dependency.

2. Had to manually create `index.html` in the project directory before running `trunk serve` because otherwise there would be this error: `Error: error getting canonical path to source HTML file "index.html"`.

3. Found [this tutorial [3]](https://yew.rs/docs/tutorial) which helped fix a bunch of bugs.

February 19, 2023

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html). You need to add `$HOME/.cargo/bin` to your `$PATH` variable in `~/.zshrc` if you're using ZSH.

2. `cargo install trunk`

3. `rustup target add wasm32-unknown-unknown`

4. `cargo new yew-video-streaming`