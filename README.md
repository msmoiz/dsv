# DSV

This is a simple library for working with delimiter-separated value (DSV) files.
For more information on usage and supported syntax, see the [crate
documentation](./src/lib.rs).

```rust
use dsv::Dsv;

let text = ["hello,sun", "hello,moon"].join("\n");
let dsv = Dsv::from_str(&text).unwrap();

assert_eq!(dsv[0][0], "hello");
assert_eq!(dsv[0][1], "sun");
assert_eq!(dsv[1][0], "hello");
assert_eq!(dsv[1][1], "moon");
```
