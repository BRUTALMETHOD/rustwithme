# Summary
1) Install all useful/pre-requisite tools like rustup-init, cargo, rust, rust-anaylzers. Running my IDE with neovim and pre-configured the LSP similar to what you get get with vscode to speed up the learning process.
https://doc.rust-lang.org/book/ch01-01-installation.html
2) Use rustc to compile a single file.
https://doc.rust-lang.org/book/ch01-02-hello-world.html
```
hj@hjs-MacBook-Air rustwithme % cd chapter1
hj@hjs-MacBook-Air chapter1 % ls
README.md       cargo           helloworld      helloworld.rs
hj@hjs-MacBook-Air chapter1 % rustc helloworld.rs
hj@hjs-MacBook-Air chapter1 % ./helloworld
Hello world
```
3) Use cargo run and cargo build for projects. Cargo helps us manage project with multiple files.
https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
```
hj@hjs-MacBook-Air chapter1 % cd cargo
hj@hjs-MacBook-Air cargo % cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
hj@hjs-MacBook-Air cargo % cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
hj@hjs-MacBook-Air cargo % cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cargo`
Hello, world!
```
