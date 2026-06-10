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
    // Could use `Default::default()` here. Until it is broken by `!1630`, funnily enough.
    DuplicateFlags::from_ord(0)
}

const SIGNALS: DuplicateFlags = DuplicateFlags::SIGNALS;
const GROUPS: DuplicateFlags = DuplicateFlags::GROUPS;
const USE_INSTANTIATION: DuplicateFlags = DuplicateFlags::USE_INSTANTIATION;

#[itest]
fn bitfield_ops_with() {
    let no_flags = no_flags();

    assert_eq!(no_flags.with(USE_INSTANTIATION), USE_INSTANTIATION);

    assert_eq!(
        GROUPS.with(SIGNALS),
        DuplicateFlags::from_ord(GROUPS.ord() | SIGNALS.ord())
    );

    assert_eq!(GROUPS.with(GROUPS), GROUPS,);

    assert_eq!(GROUPS.with(GROUPS.with(SIGNALS)), GROUPS.with(SIGNALS),);

    assert_eq!(GROUPS.with(no_flags), GROUPS);
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

    assert_eq!(GROUPS.without(no_flags), GROUPS);
}
