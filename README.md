## A lending iterator trait based on higher-rank trait bounds (HRTBs)

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

Similarly to what happens with standard iterators, besides the fundamental [`LendingIterator`] trait 
there is a [`IntoLendingIterator`] trait
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

## An example: reusing line buffers

The following code shows how to implement a lending iterator returning lines from a file,
reusing a buffer for the line:
```rust
use hrtb_lending_iterator::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Lines {
    reader: BufReader<File>,
    buffer: String,
}

impl<'any> LendingIteratorItem<'any> for Lines {
    type Type = &'any str;
}

impl LendingIterator for Lines {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        self.buffer.clear();
        if self.reader.read_line(&mut self.buffer).ok()? == 0 {
            return None;
        }
        Some(&self.buffer)
    }
}

fn main() {
    let mut iter = Lines {
        reader: BufReader::new(File::open("Cargo.toml").unwrap()),
        buffer: String::new(),
    };
    while let Some(line) = iter.next() {
        // line is a reference to the buffer
        print!("{}", line);
    }
}
```
Since the library contains several methods analogous to those of Rust iterators, you can 
enumerate just at most the first ten lines with 
```ignore
    let mut iter = Lines {
        reader: BufReader::new(File::open("Cargo.toml").unwrap()),
        buffer: String::new(),
    }.take(10);
```

Moreover, if at any time you decide that you prefer to handle owned strings, you have just
to turn the lending iterator into a standard iterator by making the returned items owned:
```ignore
    for line in iter.to_owned() {
        // line is a copy of the buffer
        print!("{}", line);
    }
```
This is possible every time that the type referenced by the returned item implements
[`ToOwned`](std::borrow::ToOwned).

## An example: overlapping windows

The following code shows how to implement a lending iterator returning overlapping windows
of a slice:
```rust
use hrtb_lending_iterator::*;

struct WindowsMut<'a, T, const WINDOW_SIZE: usize> {
    slice: &'a mut [T],
    curr_pos: usize,
}

impl<'a, 'any, T, const WINDOW_SIZE: usize> LendingIteratorItem<'any>
    for WindowsMut<'a, T, WINDOW_SIZE>
{
    type Type = &'any mut [T; WINDOW_SIZE];
}

impl<'a, T, const WINDOW_SIZE: usize> LendingIterator for WindowsMut<'a, T, WINDOW_SIZE> {
    fn next(&mut self) -> Option<Item<'_, Self>> {
        let window = self
            .slice
            .get_mut(self.curr_pos..)?
            .get_mut(..WINDOW_SIZE)?;
        self.curr_pos += 1;
        Some(window.try_into().unwrap())
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut iter = WindowsMut::<'_, _, 3> {
        slice: &mut v,
        curr_pos: 0,
    };
    while let Some(window) = iter.next() {
        // The window is mutable
        window[0] = window[2] - window[1];
        println!("{:?}", window);
    }
}
```

