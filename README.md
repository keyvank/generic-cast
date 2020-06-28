# generic-cast
Imagine you want to provide an alternative implementation for a specific type-parameter of a generic function:

```rust
use generic_cast::{cast_ref, equals};
use std::ops::Add;

fn double<T: 'static + Copy + Add<Output = T>>(a: T) -> T {
    if let Some(a) = cast_ref::<T, u32>(&a) {
        let result = a << 1; // Faster implementation
        *cast_ref::<u32, T>(&result).unwrap()
    } else {
        a.add(a).clone()
    }
}
```

Or maybe just check if the type-paramter is a specific type:
```rust
fn print<T: 'static + std::fmt::Display>(a: T) {
    if generic_cast::equals::<T, f32>() {
        println!("Its a float!");
    }
    println!("{}", a);
}
```
