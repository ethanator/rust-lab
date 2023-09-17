# rust-lab
A lab where I tinker with the Rust language.

## References

1. [YouTube | Let's Build a RUST WebAssembly Frontend App with Yew](https://www.youtube.com/watch?v=lJllt5X6ELg)

2. [Article | The current state of Rust web frameworks](https://blog.logrocket.com/current-state-rust-web-frameworks/)

3. [Docs | Yew Tutorial](https://yew.rs/docs/tutorial)

4. [Book | The Rust Programming Language](https://doc.rust-lang.org/book/ch00-00-introduction.html)

5. [YouTube | Rust and Wasm in Production](https://www.youtube.com/playlist?list=PLWtPciJ1UMuARHHLqRVzBiPZSwlqXthgN)

## Log

- September 17, 2023

    - Completed [this tutorial](https://tinyurl.com/nextjs-rust-wasm) which actually worked

    - Completed [this tutorial](https://tinyurl.com/react-rust-wasm) which did NOT work unfortunately

- March 25, 2023

    - Watched [this YouTube video](https://www.youtube.com/watch?v=7ap3CkBEpOE) which claims Leptos is faster than React on both Firefox and Chrome.

    - Watched [the YouTube video on deploying Leptos](https://www.youtube.com/watch?v=6rICMHd7as0) which brings up Yew and [Perseus](https://github.com/framesurge/perseus).

    - Watched [this YouTube video about coding a to-do list with Leptos](https://www.youtube.com/watch?v=v9rUoYX9lUU) and understood what "nightly" means in Rust with [this guide](https://rust-lang.github.io/rustup/concepts/channels.html).

- February 20, 2023

    - Added `yew` as a dependency.

    - Had to manually create `index.html` in the project directory before running `trunk serve` because otherwise there would be this error: `Error: error getting canonical path to source HTML file "index.html"`.

    - Found [this tutorial [3]](https://yew.rs/docs/tutorial) which helped fix a bunch of bugs.

- February 19, 2023

    - Installed Rust and Cargo following [this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html). You need to add `$HOME/.cargo/bin` to your `$PATH` variable in `~/.zshrc` if you're using ZSH.

    - `cargo install trunk`

    - `rustup target add wasm32-unknown-unknown`

    - `cargo new yew-video-streaming`
