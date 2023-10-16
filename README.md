## A lending iterator based on generic associated types and higher-rank trait bounds

A *lending iterator* is an iterator which lends mutable borrows to the items it returns.
In particular, this means that the reference to an item is invalidated by the 
next call to `next()`. The typical example that cannot
be written with standard Rust iterators, but is covered by lending iterator,
is that of an iterator returning mutable, overlapping windows
of a slice. But lending iterators are more general than that, as they
might return items that depend on some mutable state stored in the iterator. For example,
starting from an iterator on pairs of integers lexicographically sorted, a lending iterator might return
iterators on pairs with the same first coordinate without any copying; clearly, any call on
`next()` would invalidate the reference returned by the previous call.

The design of the lending iterator contained in this library was proposed by
[Daniel Henry Mantilla](https://github.com/danielhenrymantilla/lending-iterator.rs/issues/13) and
is similar to the design of his popular [lending-iterator](https://crates.io/crates/lending-iterator) 
crate, but it uses generic associated types;
also [Yoav Tzfati](https://github.com/Crazytieguy/gat-lending-iterator/issues/12) participated to
the discussion, providing information and code.

Note that the design is significantly more complex than the “obvious” lending iterator
```rust
pub trait LendingIterator {
    type Item<'b> where Self: 'b;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```
The previous design proved to be too restrictive, and would have made it impossible to
write types such as `PermutedGraph` or `PairsGraph` in 
[the Rust port of webgraph](https://github.com/vigna/webgraph-rs/).

