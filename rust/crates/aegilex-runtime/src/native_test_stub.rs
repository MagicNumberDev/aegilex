// The test-only native target supplies legacy host symbols while the C ABI is migrated.
#[link(name = "aegilex_test_stub", kind = "static")]
unsafe extern "C" {}
