/*
 * SPDX-FileCopyrightText: 2023 Inria
 * SPDX-FileCopyrightText: 2023 Tommaso Fontana
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::adapters::*;

/// A trait specifying the type of the items of a [LendingIterator].
///
/// Note that the trait specifies that `Self` must outlive `'any`
/// in a way that is inherited by implementations.
pub trait LendingIteratorItem<'any, WhereSelfOutlivesAny = &'any Self> {
    type Type;
}

/// A readable shorthand for the type of the items of a [`LendingIterator`] `I`.
pub type Item<'any, I> = <I as LendingIteratorItem<'any>>::Type;

/**

The main trait: an iterator that borrows its items mutably from
`self`, which implies that you cannot own at the same time two returned
items.

The trait depends on the trait [LendingIteratorItem], which specifies the
type of items returned by the iterator, via higher-kind trait bounds.

This design was proposed
[Daniel Henry Mantilla](https://github.com/danielhenrymantilla/lending-iterator.rs/issues/13) and
is similar to the design of his popular [lending-iterator](https://crates.io/crates/lending-iterator)
crate, but it uses directly higher-rank trait bounds, rather than simulating them with macros.

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


*/

pub trait LendingIterator: for<'any> LendingIteratorItem<'any> {
    fn next(&mut self) -> Option<Item<'_, Self>>;

    /// Like [`Iterator::take`], creates an iterator that yields the first `n` elements,
    /// or fewer if the underlying iterator ends sooner.
    fn take(self, n: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take {
            iter: self,
            remaining: n,
        }
    }

    /// Like [`Iterator::take_while`], creates an iterator that yields elements based
    /// on a predicate.
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&'_ Item<'_, Self>) -> bool,
    {
        TakeWhile {
            iter: self,
            predicate,
            ended: false,
        }
    }

    /// Like [`Iterator::inspect`], does something with each element of an iterator,
    /// passing the value on.
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        for<'any> F: FnMut(&'_ Item<'_, Self>),
    {
        Inspect { iter: self, f }
    }

    /// Like [`Iterator::map`], takes a closure and creates an iterator which calls
    /// that closure on each element.
    fn map<NewItemType, F>(self, map: F) -> Map<Self, F, NewItemType>
    where
        Self: Sized,
        for<'any> F: FnMut(Item<'_, Self>) -> NewItemType,
    {
        Map { iter: self, map }
    }

    /// Like [`Iterator::fold`], folds every element into an accumulator by applying
    /// an operation, returning the final result.
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Item<'_, Self>) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }

    /// Like [`Iterator::for_each`], calls a closure on each element of an iterator.
    fn for_each(self, mut f: impl FnMut(Item<'_, Self>))
    where
        Self: Sized,
    {
        self.fold((), |(), item| f(item))
    }

    /// Turns this [`LendingIterator`] into a regular [`Iterator`],
    /// if possible, without allocating.
    ///
    /// This method is only available if the items returned
    /// by the iterator are owned (i.e., if the iterator is
    /// not really lending).
    ///
    /// Note that this method and [`crate::IteratorExt::into_lend_iter`] are
    /// mutually inverse.
    fn into_iter<Item>(self) -> IntoIter<Self>
    where
        Self: for<'any> LendingIteratorItem<'any, Type = Item>,
        Self: Sized,
    {
        IntoIter(self)
    }

    /// Turns this [`LendingIterator`] into a regular [`Iterator`]
    /// by getting an owned version of the returned items via
    /// [`ToOwned`].
    ///
    /// This method is only available if the type referred by
    /// the item type implements [`ToOwned`].
    fn to_owned(self) -> ToOwnedItemIterator<Self>
    where
        Self: Sized,
    {
        ToOwnedItemIterator(self)
    }

    /// Like [`Iterator::enumerate`], creates an iterator which gives the current
    /// iteration count as well as the next value.
    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate::new(self)
    }
}

/// This struct is returned by [`LendingIterator::take`]
#[derive(Clone, Debug)]
pub struct Take<I: LendingIterator> {
    pub(crate) iter: I,
    pub(crate) remaining: usize,
}

impl<'any, I: LendingIterator> LendingIteratorItem<'any> for Take<I> {
    type Type = <I as LendingIteratorItem<'any>>::Type;
}

impl<I: LendingIterator> LendingIterator for Take<I> {
    fn next(&'_ mut self) -> Option<Item<'_, I>> {
        if self.remaining > 0 {
            self.remaining -= 1;
            self.iter.next()
        } else {
            None
        }
    }
}
