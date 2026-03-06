mod common;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/implementation-plan.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-implementation-plan.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-implementation-plan.yaml"
);


// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_implementation_plan() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_implementation_plan_missing_assumptions() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "missing assumptions should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_title() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "summary: A summary\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing title should be rejected");
}

#[test]
fn rejects_missing_summary() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "title: A plan\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing summary should be rejected");
}

#[test]
fn rejects_missing_key_changes() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "title: A plan\nsummary: A summary\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing key-changes should be rejected");
}

#[test]
fn rejects_missing_test_strategy() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "title: A plan\nsummary: A summary\nkey-changes:\n  - change one\nassumptions:\n  - assumption one\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing test-strategy should be rejected"
    );
}

// ── Additional properties allowed ───────────────────────────────

#[test]
fn accepts_extra_fields_at_top_level() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "title: A plan\nsummary: A summary\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\ncustom-field: extra data\n",
    );
    assert!(
        validator.is_valid(&instance),
        "extra fields should be accepted (no additionalProperties: false)"
    );
}
