use std::fs;

/// Read a test fixture from `tests/fixtures/`.
pub fn read_test_fixture(name: &str) -> String {
    fs::read_to_string(format!("tests/fixtures/{}", name))
        .unwrap_or_else(|e| panic!("fixture tests/fixtures/{} should exist: {}", name, e))
}
