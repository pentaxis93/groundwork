use jsonschema::Validator;
use serde_json::Value;

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

fn load_schema() -> Validator {
    let text = std::fs::read_to_string(SCHEMA_PATH).expect("read schema");
    let value: Value = serde_json::from_str(&text).expect("parse schema JSON");
    Validator::new(&value).expect("compile schema")
}

fn yaml_to_json(yaml: &str) -> Value {
    serde_yml::from_str(yaml).expect("parse YAML")
}

// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_implementation_plan() {
    let validator = load_schema();
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_implementation_plan_missing_assumptions() {
    let validator = load_schema();
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "missing assumptions should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_title() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "summary: A summary\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing title should be rejected");
}

#[test]
fn rejects_missing_summary() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A plan\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing summary should be rejected");
}

#[test]
fn rejects_missing_key_changes() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A plan\nsummary: A summary\ntest-strategy: Run tests\nassumptions:\n  - assumption one\n",
    );
    assert!(!validator.is_valid(&instance), "missing key-changes should be rejected");
}

#[test]
fn rejects_missing_test_strategy() {
    let validator = load_schema();
    let instance = yaml_to_json(
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
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A plan\nsummary: A summary\nkey-changes:\n  - change one\ntest-strategy: Run tests\nassumptions:\n  - assumption one\ncustom-field: extra data\n",
    );
    assert!(
        validator.is_valid(&instance),
        "extra fields should be accepted (no additionalProperties: false)"
    );
}
