## A lending iterator

The design of the lending iterator contained in this library was proposed by
[Daniel Henry Mantilla](https://github.com/danielhenrymantilla/lending-iterator.rs/issues/13);
also [Yoav Tzfati](https://github.com/Crazytieguy/gat-lending-iterator/issues/12) participated to
the discussion, providing information and code.

Note that the design is significantly more complex than
```rust
pub trait LendingIterator {
    type Item<'b> where Self: 'b;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```
However, the previous design proved to be too restrictive, and would have made it impossible to
write types such as `PermutedGraph` or `PairsGraph` in [the Rust port of webgraph](https://github.com/vigna/webgraph-rs/).

