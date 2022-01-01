# click-once

A small tiny little binary to fix undesired mouse double clicks in Windows, written in Rust. Minimal executable with little to no overhead.

In my machine, CPU usage does not exceed 0.15% and RAM usage is ~700kB

# How it works

It basically hijacks a global hook into the Windows's low level mouse thread input queue and rejects mouse releases which happen too quickly right after a mouse down input.


# Run
```
./click-once.exe <delay_left_but> <delay_right_but> 
```

`delay`s are in ms and can be adjusted. The default is 28ms for `delay_left_but` and 0 for `delay_right_but` (disabled).

# Build

- Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repo and build with Cargo

```
git clone https://github.com/j-hc/click-once
cd click-once
cargo build --release
```
