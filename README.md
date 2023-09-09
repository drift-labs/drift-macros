# drift macros
misc helpful macros 

## #[assert_no_slop]

throws compile-time error if repr(C) struct has hidden padding (slop)

```rust
#[assert_no_slop]
#[account(zero_copy)]
#[repr(C)]
pub struct Foo {
    pub bar: u128,
    pub baz: u8,
    pub qux: u128,
}
```

this will throw a compile-time error because `Foo` has 7 bytes of hidden padding between `baz` and `qux`