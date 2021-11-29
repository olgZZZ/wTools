# module::implements

Macro to answer the question: does it implement a trait?

This solution has a limitation:

- In case enity is a function and trat is `Fn`/`FnMut`/`FnOnce` which current entity does not implement you will get compile-time error instead of `false`.

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/meta_implements_trivial
cargo run
```

### To add to your project

```
cargo add implements
```

### Sample

``` rust sample test
use implements::*;

dbg!( implements!( 13_i32 => Copy ) );
// < implements!( 13_i32 => Copy ) : true
dbg!( implements!( Box::new( 13_i32 ) => Copy ) );
// < implements!( 13_i32 => Copy ) : false
```