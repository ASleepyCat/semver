#![allow(clippy::extra_unused_type_parameters)]

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test() {
    assert_send_sync::<tf_semver::BuildMetadata>();
    assert_send_sync::<tf_semver::Comparator>();
    assert_send_sync::<tf_semver::Error>();
    assert_send_sync::<tf_semver::Prerelease>();
    assert_send_sync::<tf_semver::Version>();
    assert_send_sync::<tf_semver::VersionReq>();
    assert_send_sync::<tf_semver::Op>();
}
