# queen-sweep-macros

This crate only exists because I wanted to try making procedural macros :P

## `#[heuristic]`

Annotate a function to enforce the correct heuristic function signature at compile time.

```rust
#[heuristic]
fn my_heuristic(ctx: &HeuristicContent) -> Vec<((usize, usize), f32)> {
    /* ... */
}
```
is expanded into
```rs
fn my_heuristic(ctx: &HeuristicContent) -> Vec<((usize, usize), f32)> {
    /* ... */
}
const _: HeuristicFn = my_heuristic;
```
