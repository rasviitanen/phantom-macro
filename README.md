# phantom_macro
A macro used for expanding token trees but where you don't
want to use the invariant from the expansion.

## Example
```rust
use phantom_macro::expand;

macro_rules! csv {
    [$($item: literal),* $(,)?] => {
        format!(
            concat!($(expand!($item => "{},")),*),
            $($item),*
        )
    }
}

fn main() {
    let csv = csv!["a", "b", "c"];
}
```

## Why
Rust will not accept expansions where you don't use the invariants.
With this macro we can trick the compiler and expand trees without including
the invariant in the final expansion.