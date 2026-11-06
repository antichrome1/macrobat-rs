# macrobat

Define multiple single-arm rust declarative macros in a familiar
function-like syntax.

## Example Usage

Add `macrobat = "0.9.0"` to your dev dependencies in Cargo.toml.

my_macros.rs:
```rust
use macrobat::macros;

macros! {
    macro add($x:expr, $y:expr) {
        $x + $y
    } @export

    macro echo($x:expr) {
        // do some computation
        let _gg = 11;
        // ...

        // return this value
        $x
    } @export @wrap
}
```

main.rs:
```rust
#[path = "my_macros.rs"]
pub mod my_macros;

fn main() {
    println!("{}", add!(1, 2));
    println!("{}", echo!(1));
}
```

## License, AI disclosure

No AI agents/tools were/are used in this project.

Copyright (c) 2026 Aditya Singh <antichrome1@proton.me>

SPDX-License-Identifier: ISC
