# macrobat

This library aims to reduce the cognitive overhead of reading and
writing single-arm declarative macros, so you can spend your energy on
what your macro actually does.

With macrobat, you can define multiple single-arm declarative macros
in a familiar function-like syntax like so:

```rust
macros! {
     
    macro fib($x:expr) {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..n {
            (a, b) = (b, a + b);
        }
        a
    } @export @wrap

    // add more macros here

}

// Call the macro like you usually would
fib!(10);
```

## License

ISC
