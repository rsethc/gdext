use godot::classes::node::DuplicateFlags;

#[test]
fn bitfield_ops_with() {
    // Test that adding a flag to nothing, with `with` function,
    // gives exactly that flag.
    assert_eq!(
        DuplicateFlags::from_ord(0).with(DuplicateFlags::USE_INSTANTIATION),
        DuplicateFlags::USE_INSTANTIATION
    );

    // Test adding another flag to an existing one.
    // TO DO

    // Test adding flags which are already present,
    // which should result in no change.
    // TO DO
    
    // Test that when adding a flag, where some are
    // already present, the ones that are supposed to
    // be added do in fact get added.
    // TO DO
}

#[test]
fn bitfield_ops_without() {
    // Test that removing a flag from itself with `without`
    // gives no flags.
    assert_eq!(
        DuplicateFlags::USE_INSTANTIATION.without(DuplicateFlags::USE_INSTANTIATION),
        DuplicateFlags::from_ord(0)
    );

    // Test removing a flag from several.
    // TO DO
    
    // Test removing a flag that does not exist. The result
    // should be unchanged.
    // TO DO

    // Test removing several flags, where some were present
    // and others were not. The ones that were present should
    // be removed.
    // TO DO
}
