mod common;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/completion-record.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-completion-record.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-completion-record.yaml"
);

// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_completion_record() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_completion_record_missing_gaps() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "missing gaps field should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_behavior_coverage_summary() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "gaps: []\nmerge-reference: \"PR #1\"\nclosure-confirmation: Done\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing behavior-coverage-summary should be rejected"
    );
}

#[test]
fn rejects_missing_merge_reference() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage-summary: All passing\ngaps: []\nclosure-confirmation: Done\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing merge-reference should be rejected"
    );
}

#[test]
fn rejects_missing_closure_confirmation() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage-summary: All passing\ngaps: []\nmerge-reference: \"PR #1\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing closure-confirmation should be rejected"
    );
}

// ── Valid with empty gaps ───────────────────────────────────────

#[test]
fn accepts_empty_gaps_array() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage-summary: All passing\ngaps: []\nmerge-reference: \"PR #1\"\nclosure-confirmation: Done\n",
    );
    assert!(
        validator.is_valid(&instance),
        "empty gaps array should be accepted"
    );
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage-summary: All passing\ngaps: []\nmerge-reference: \"PR #1\"\nclosure-confirmation: Done\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}
