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
fn no_flags () -> DuplicateFlags {
    // Could use `Default::default()` here.
    // Until it is broken by `!1630`, funnily enough.

    DuplicateFlags::from_ord(0)
}

const SIGNALS: DuplicateFlags = DuplicateFlags::SIGNALS;
const GROUPS: DuplicateFlags = DuplicateFlags::GROUPS;
const USE_INSTANTIATION: DuplicateFlags = DuplicateFlags::USE_INSTANTIATION;

#[itest]
fn bitfield_ops_with() {
    let no_flags = no_flags();

    // Test that adding a flag to nothing, with `with` function,
    // gives exactly that flag.
    assert_eq!(
        no_flags.with(USE_INSTANTIATION),
        USE_INSTANTIATION
    );

    // Test adding another flag to an existing one.
    assert_eq!(
        GROUPS.with(SIGNALS),
        DuplicateFlags::from_ord(GROUPS.ord() | SIGNALS.ord())
    );

    // Test adding flags which are already present,
    // which should result in no change.
    assert_eq!(
        GROUPS.with(GROUPS),
        GROUPS,
    );

    // Test that when adding a flag, where some are
    // already present, the ones that are supposed to
    // be added do in fact get added.
    assert_eq!(
        GROUPS.with(GROUPS.with(SIGNALS)),
        GROUPS.with(SIGNALS),
    );

    // Test that adding no flags at all is successful
    // and does not result in any change.
    assert_eq!(
        GROUPS.with(no_flags),
        GROUPS
    );
}

#[itest]
fn bitfield_ops_without() {
    let no_flags = no_flags();

    // Test that removing a flag from itself with `without`
    // gives no flags.
    assert_eq!(
        USE_INSTANTIATION.without(USE_INSTANTIATION),
        no_flags
    );

    // Test removing a flag from several.
    assert_eq!(
        SIGNALS
            .with(GROUPS)
            .without(SIGNALS),
        GROUPS
    );

    // Test removing a flag that does not exist. The result
    // should be unchanged.
    assert_eq!(
        GROUPS.without(SIGNALS),
        GROUPS
    );

    // Test removing several flags, where some were present
    // and others were not. The ones that were present should
    // be removed.
    assert_eq!(
        SIGNALS
            .with(GROUPS)
            .without(SIGNALS.with(USE_INSTANTIATION)),
        GROUPS
    );

    // Test that removing no flags at all is successful
    // and does not result in any change.
    assert_eq!(
        GROUPS.without(no_flags),
        GROUPS
    );
}
