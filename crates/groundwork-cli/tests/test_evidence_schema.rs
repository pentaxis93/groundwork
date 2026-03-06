mod common;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/test-evidence.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-test-evidence.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-test-evidence.yaml"
);

// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_test_evidence() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_test_evidence_bad_result_value() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "invalid result value 'success' should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_evidence() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json("{}");
    assert!(!validator.is_valid(&instance), "missing evidence should be rejected");
}

#[test]
fn rejects_evidence_entry_missing_scenario() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - result: pass\n    command: cargo test\n    output-summary: passed\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "evidence entry missing scenario should be rejected"
    );
}

#[test]
fn rejects_evidence_entry_missing_result() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - scenario: test scenario\n    command: cargo test\n    output-summary: passed\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "evidence entry missing result should be rejected"
    );
}

#[test]
fn rejects_evidence_entry_missing_command() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - scenario: test scenario\n    result: pass\n    output-summary: passed\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "evidence entry missing command should be rejected"
    );
}

#[test]
fn rejects_evidence_entry_missing_output_summary() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - scenario: test scenario\n    result: pass\n    command: cargo test\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "evidence entry missing output-summary should be rejected"
    );
}

// ── Array constraint rejections ─────────────────────────────────

#[test]
fn rejects_empty_evidence() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json("evidence: []\n");
    assert!(!validator.is_valid(&instance), "empty evidence should be rejected");
}

// ── Enum rejections ─────────────────────────────────────────────

#[test]
fn rejects_invalid_result_values() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["success", "failure", "error", "skip", "PASS"] {
        let yaml = format!(
            "evidence:\n  - scenario: test scenario\n    result: {bad}\n    command: cargo test\n    output-summary: output\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "result {:?} should be rejected",
            bad
        );
    }
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - scenario: test scenario\n    result: pass\n    command: cargo test\n    output-summary: passed\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_evidence_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "evidence:\n  - scenario: test scenario\n    result: pass\n    command: cargo test\n    output-summary: passed\n    duration: 1.5s\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in evidence entry should be rejected"
    );
}
