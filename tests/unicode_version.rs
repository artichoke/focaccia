#[test]
fn public_unicode_version_matches_bundled_data() {
    let (major, minor, patch) = focaccia::UNICODE_VERSION;
    let header = format!("# CaseFolding-{major}.{minor}.{patch}.txt");
    assert!(include_str!("../CaseFolding.txt").starts_with(&header));
}
