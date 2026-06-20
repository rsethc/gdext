/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::node::DuplicateFlags;
use godot::obj::EngineBitfield;

use crate::framework::itest;

/// Necessarily since `from_ord` is not `const`.
fn no_flags() -> DuplicateFlags {
    // Avoid `Default::default()` as its value and presence might change in a future version.
    DuplicateFlags::from_ord(0)
}

const SIGNALS: DuplicateFlags = DuplicateFlags::SIGNALS; // 1
const GROUPS: DuplicateFlags = DuplicateFlags::GROUPS;   // 2
const SCRIPTS: DuplicateFlags = DuplicateFlags::SCRIPTS; // 4
    
#[itest]
fn bitfield_ops_with() {
    let no_flags = DuplicateFlags::from_ord(0);

    assert_eq!(no_flags.with(GROUPS).ord(), 2);
    assert_eq!(GROUPS.with(no_flags).ord(), 2);
    assert_eq!(GROUPS.with(GROUPS).ord(), 2);

    assert_eq!(GROUPS.with(SIGNALS).ord(), 1 | 2);
    assert_eq!(GROUPS.with(GROUPS.with(SIGNALS)).ord(), 1 | 2);
    assert_eq!(GROUPS.with(GROUPS).with(SIGNALS).ord(), 1 | 2);

    assert_eq!(GROUPS.with(SIGNALS).with(SCRIPTS).ord(), 1 | 2 | 4);
}

#[itest]
fn bitfield_ops_without() {
    let no_flags = no_flags();

    assert_eq!(USE_INSTANTIATION.without(USE_INSTANTIATION), no_flags);

    assert_eq!(SIGNALS.with(GROUPS).without(SIGNALS), GROUPS);

    assert_eq!(GROUPS.without(SIGNALS), GROUPS);

    assert_eq!(
        SIGNALS
            .with(GROUPS)
            .without(SIGNALS.with(USE_INSTANTIATION)),
        GROUPS
    );

    let without_then_without = GROUPS
        .with(SIGNALS)
        .with(USE_INSTANTIATION)
        .without(SIGNALS)
        .without(USE_INSTANTIATION);
    assert!(without_then_without.is_set(GROUPS));
    assert!(!without_then_without.is_set(SIGNALS));
    assert!(!without_then_without.is_set(USE_INSTANTIATION));

    assert_eq!(GROUPS.without(no_flags), GROUPS);
}
