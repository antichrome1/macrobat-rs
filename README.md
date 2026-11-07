# macrobat

Define multiple single-arm rust declarative macros in a familiar
function-like syntax.

- `@wrap` wraps your code block in braces, which is useful when you
  have multiple statements to do some computation and then return a
  value.
- `@export` adds `#[macro_export]` attr to your macro.

## Example Usage

my_macros.rs:
```rust
use macrobat::macros;

macros! {

    macro add($x:expr, $y:expr) {
        $x + $y
    } @export

    macro collatz_steps($x:expr) {
        let mut n = $x;
        let mut steps = 0u64;
        while n > 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            steps += 1;
        }
        steps
    } @export @wrap

}
```

main.rs:
```rust
#[path = "my_macros.rs"]
pub mod my_macros;

fn main() {
    println!("{}", add!(1, 2));
    println!("{}", collatz_steps!(27));
}
```

## Metadata

License: ISC
Contact: <antichrome1@proton.me>
