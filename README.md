## A lending iterator based on generic associated types and higher-rank trait bounds (HRTBs)

A *lending iterator* is an iterator which lends mutable borrows to the items it returns.
In particular, this means that the reference to an item is invalidated by the 
next call to `next()`.

The typical example that cannot
be written with standard Rust iterators, but is covered by lending iterators,
is that of an iterator returning mutable, overlapping windows
of a slice.

But lending iterators are more general than that, as they
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
    type Item<'a> where Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}
```
The previous design proved to be too restrictive, as it would have made it impossible to
write types such as `PermutedGraph` or `ArcListGraph` in 
[the Rust port of WebGraph](https://github.com/vigna/webgraph-rs/).

Similarly to what happens with standard iterators, there is a [`IntoLendingIterator`] trait
and methods such as [`LendingIterator::map`]. Our aim is to have a library as complete as that
of standard iterators, but there is still a lot of work to do.

The Rust syntax for iterating over types implementing [`IntoIterator`] cannot be extended
to lending iterators. The idiomatic way of iterate over a lending iterator is to use
a `while let` loop, as in:
```ignore
while let Some(item) = iter.next() {
    // Do something with item
}
```
Note that if you have a variable `x` with an `iter` method returning a lending iterator,
you cannot use the form `while let Some(item) = x.iter().next()` as you will iterate
over the first element forever.

To make iteration simpler, we provide a macro [`for_lend!`] that can be used to iterate in a
way more similar to a `for` loop.

## Type-inference problems

Note that due to the complex type dependencies, generic associated
types and higher-kind trait bounds, the current Rust compiler cannot
always infer the correct type of the associated iterator type
and of the items it returns.
In general, when writing methods accepting an [`IntoLendingIterator`]
restricting the returned item with a *type* will work, as in:

```rust
use hrtb_lending_iterator::*;

struct MockLendingIterator {}

impl<'any> LendingIteratorItem<'any> for MockLendingIterator {
    type Type = &'any str;
}

impl LendingIterator for MockLendingIterator {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        None
    }
}

fn read_lend_iter<L>(iter: L)
where
    L: LendingIterator + for<'any> LendingIteratorItem<'any, Type = &'any str>,
{}

fn test_mock_lend_iter(m: MockLendingIterator) {
    read_lend_iter(m);
}
```

However, the following code, which restricts the returned items using a trait bound,
does not compile as of Rust 1.73.0:

```ignore
use hrtb_lending_iterator::*;

struct MockLendingIterator {}

impl<'any> LendingIteratorItem<'any> for MockLendingIterator {
    type Type = &'any str;
}

impl LendingIterator for MockLendingIterator {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        None
    }
}

fn read_lend_iter<L>(iter: L)
where
    L: LendingIterator,
    for<'any> <L as LendingIteratorItem<'any>>::Type: AsRef<str>,
{}

fn test_mock_lend_iter(m: MockLendingIterator) {
    read_lend_iter(&m);
}
```

The workaround is to use an explicit type annotation:

```rust
use hrtb_lending_iterator::*;

struct MockLendingIterator {}

impl<'any> LendingIteratorItem<'any> for MockLendingIterator {
    type Type = &'any str;
}

impl LendingIterator for MockLendingIterator {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        None
    }
}

fn read_lend_iter<L>(iter: L)
where
    L: LendingIterator,
    for<'any> <L as LendingIteratorItem<'any>>::Type: AsRef<str>,
{}

fn test_mock_lend_iter(m: MockLendingIterator) {
    read_lend_iter::<MockLendingIterator>(m);
}
```
