# fn-compose

A small macro for function composition.

## Usage

```rust
use fn_compose::fn_compose;
let res1 = fn_compose!(1 => add_one => double => mul 3, ;, 5);
let res2 = mul(3, double(add_one(1)), 5);
assert_eq!(res1, res2);
```
