/*
 * SPDX-FileCopyrightText: 2023 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use hrtb_lending_iterator::*;

#[test]
fn test_windows_mut() {
    let mut v = [0, 1, 2, 3, 4];

    for_lend! {w in v.windows_mut::<3>().enumerate() =>
        let (i, w) = w;
        assert_eq!(w.as_ref(), [i, i + 1, i + 2]);
        w[0] = w[2] - w[1];
    }
    assert_eq!(v[..3], [1, 1, 1]);
}
