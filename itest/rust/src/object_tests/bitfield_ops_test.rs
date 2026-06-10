/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::{classes::node::DuplicateFlags, obj::EngineBitfield};
use crate::framework::itest;

#[itest]
fn bitfield_ops_with() {
    // Test that adding a flag to nothing, with `with` function,
    // gives exactly that flag.
    assert_eq!(
        DuplicateFlags::from_ord(0).with(DuplicateFlags::USE_INSTANTIATION),
        DuplicateFlags::USE_INSTANTIATION
    );

    // Test adding another flag to an existing one.
    assert_eq!(
        DuplicateFlags::GROUPS.with(DuplicateFlags::SIGNALS),
        DuplicateFlags::from_ord(
            DuplicateFlags::GROUPS.ord() | DuplicateFlags::SIGNALS.ord()
        )
    );

    // Test adding flags which are already present,
    // which should result in no change.
    assert_eq!(
        DuplicateFlags::GROUPS.with(
            DuplicateFlags::GROUPS
        ),
        DuplicateFlags::GROUPS,
    );
    
    // Test that when adding a flag, where some are
    // already present, the ones that are supposed to
    // be added do in fact get added.
    assert_eq!(
        DuplicateFlags::GROUPS.with(
            DuplicateFlags::GROUPS.with(DuplicateFlags::SIGNALS)
        ),
        DuplicateFlags::GROUPS.with(DuplicateFlags::SIGNALS),
    );

    // Test that adding no flags at all is successful
    // and does not result in any change.
    assert_eq!(
        DuplicateFlags::GROUPS.with(DuplicateFlags::from_ord(0)),
        DuplicateFlags::GROUPS
    );
}

#[itest]
fn bitfield_ops_without() {
    // Test that removing a flag from itself with `without`
    // gives no flags.
    assert_eq!(
        DuplicateFlags::USE_INSTANTIATION.without(DuplicateFlags::USE_INSTANTIATION),
        DuplicateFlags::from_ord(0)
    );

    // Test removing a flag from several.
    assert_eq!(
        DuplicateFlags::SIGNALS
            .with(DuplicateFlags::GROUPS)
            .without(DuplicateFlags::SIGNALS),
        DuplicateFlags::GROUPS
    );
    
    // Test removing a flag that does not exist. The result
    // should be unchanged.
    assert_eq!(
        DuplicateFlags::GROUPS
            .without(DuplicateFlags::SIGNALS),
        DuplicateFlags::GROUPS
    );

    // Test removing several flags, where some were present
    // and others were not. The ones that were present should
    // be removed.
    assert_eq!(
        DuplicateFlags::SIGNALS
            .with(DuplicateFlags::GROUPS)
            .without(
                DuplicateFlags::SIGNALS
                    .with(DuplicateFlags::USE_INSTANTIATION)
            ),
        DuplicateFlags::GROUPS
    );

    // Test that removing no flags at all is successful
    // and does not result in any change.
    assert_eq!(
        DuplicateFlags::GROUPS
            .without(DuplicateFlags::from_ord(0)),
        DuplicateFlags::GROUPS
    );
}
