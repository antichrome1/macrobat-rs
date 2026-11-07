# macrobat

Define multiple single-arm rust declarative macros in a familiar
function-like syntax.

`macrobat` aims to reduce the cognitive overhead of reading and
writing single arm declarative macros.

## Example Usage

```rust
use macrobat::macros;

macros! {

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

assert_eq!(111, collatz_steps!(27));
```

## Metadata

License: ISC <br>
Contact: <antichrome1@proton.me> <br>
Repository: https://github.com/antichrome1/macrobat-rs
