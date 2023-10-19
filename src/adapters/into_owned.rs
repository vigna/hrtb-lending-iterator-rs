/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::{LendingIterator, LendingIteratorItem};
use std::borrow::ToOwned;

/// This struct is returned by [`LendingIterator::to_owned`]
#[derive(Clone, Debug)]
pub struct IntoOwned<I: ?Sized + LendingIterator>(pub(crate) I)
where
    for<'any> <I as LendingIteratorItem<'any>>::Type: ToOwned;

impl<Item: ToOwned, I: ?Sized + LendingIterator> Iterator for IntoOwned<I>
where
    I: for<'any> LendingIteratorItem<'any, Type = Item>,
{
    type Item = Item::Owned;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.to_owned())
    }
}

#[test]
fn test_to_owned() {
    use crate::*;

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
        L: for<'any> LendingIteratorItem<'any, Type = &'any str>,
    {
        let _i = iter.to_owned();
        /*         let a = i.next();
        let b = i.next();*/
    }
}
