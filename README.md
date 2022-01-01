# click-once

A small tiny little binary to fix malfunctioning mouse double clicks in Windows, written in Rust. Minimal executable with little to no overhead.

# How it works

It basically hijacks a global hook into the Windows's low level mouse thread input queue and rejects mouse releases which happen too quickly right after a mouse down input.


# Run
```
./click-once.exe <delay>
```

\<delay\> is in ms and can be adjusted. The default is 28ms.

# Build

- Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Cargo build

```
cargo build --release
```

