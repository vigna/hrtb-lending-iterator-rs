/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use hrtb_lending_iterator::{IntoIteratorExt, IntoLendingIterator, IteratorExt, LendingIterator};

#[test]
fn test_from_into() {
    let v = [0, 1, 2, 3, 4];

    let w = v.into_into_lend_iter().into_into_iter().into_iter();
    assert_eq!(
        w.into_iter().collect::<Vec<_>>(),
        v.into_iter().collect::<Vec<_>>()
    );

    let w = v.into_iter().into_lend_iter().into_iter();
    assert_eq!(w.collect::<Vec<_>>(), v.into_iter().collect::<Vec<_>>());
}
