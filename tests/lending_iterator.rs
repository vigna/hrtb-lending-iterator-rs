/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use hrtb_lending_iterator::*;

#[test]
fn test_take() {
    let v = [0, 1, 2, 3, 4];
    let iter = v.into_into_lend_iter().into_lend_iter().take(3);
    assert_eq!(iter.into_iter().collect::<Vec<_>>(), [0, 1, 2]);

    let mut iter = v.into_into_lend_iter().into_lend_iter();
    iter.next();
    assert_eq!(iter.take(3).into_iter().collect::<Vec<_>>(), [1, 2, 3]);

    let mut iter = v.into_into_lend_iter().into_lend_iter();
    iter.next();
    assert_eq!(iter.take(5).into_iter().collect::<Vec<_>>(), [1, 2, 3, 4]);
}

#[test]
fn test_inspect() {
    let v = [0, 1, 2, 3, 4];
    let mut c = 0;
    let iter = v.into_into_lend_iter().into_lend_iter().inspect(|_| c += 1);
    let _ = iter.into_iter().collect::<Vec<_>>();
    assert_eq!(c, 5);
}

#[test]
fn test_map() {
    let v = [0, 1, 2, 3, 4];
    let iter = v.into_into_lend_iter().into_lend_iter().map(|x| x + 1);
    assert_eq!(iter.into_iter().collect::<Vec<_>>(), [1, 2, 3, 4, 5]);
}

#[test]
fn test_take_while() {
    let v = [0, 1, 2, 3, 4];
    let iter = v
        .into_into_lend_iter()
        .into_lend_iter()
        .take_while(|x| *x < 3);
    assert_eq!(iter.into_iter().collect::<Vec<_>>(), [0, 1, 2]);
}

#[test]
fn test_to_owned_item_slice() {
    let v = [0, 1, 2, 3, 4];
    let mut iter = v.windows(2).into_lend_iter().to_owned_item();
    //let a = iter.next();
    //let b = iter.next();
}

#[test]
fn test_to_owned_item_string() {
    use std::io::BufRead;

    struct Lines<B: BufRead> {
        reader: B,
        buffer: String,
    }

    impl<'any, B: BufRead> LendingIteratorItem<'any> for Lines<B> {
        type Type = &'any str;
    }

    impl<R: BufRead> LendingIterator for Lines<R> {
        fn next(&mut self) -> Option<Item<'_, Self>> {
            self.buffer.clear();
            if self.reader.read_line(&mut self.buffer).unwrap() == 0 {
                return None;
            }
            Some(&self.buffer)
        }
    }

    let mut iter = Lines {
        reader: "foo\nbar\nbaz".as_ref(),
        buffer: String::new(),
    }
    .to_owned_item();
    let a = iter.next();
    let b = iter.next();
    let c = iter.next();
    assert_eq!(Some("foo\n".to_owned()), a);
    assert_eq!(Some("bar\n".to_owned()), b);
    assert_eq!(Some("baz".to_owned()), c);
    assert_eq!(iter.next(), None);
}
