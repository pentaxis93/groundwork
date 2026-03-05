use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/completion-evidence.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-completion-evidence.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-completion-evidence.yaml"
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
fn valid_completion_evidence() {
    let validator = load_schema();
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_completion_evidence_uppercase_artifact_name() {
    let validator = load_schema();
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "uppercase artifact name should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_behavior_coverage() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "review-status:\n  review-record: review-record.yaml\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing behavior-coverage should be rejected"
    );
}

#[test]
fn rejects_missing_review_status() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "behavior-coverage:\n  test-evidence: test-evidence.yaml\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing review-status should be rejected"
    );
}

#[test]
fn rejects_missing_documentation_status() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "behavior-coverage:\n  test-evidence: test-evidence.yaml\nreview-status:\n  review-record: review-record.yaml\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing documentation-status should be rejected"
    );
}

// ── Pattern rejections ──────────────────────────────────────────

#[test]
fn rejects_invalid_test_evidence_artifact_name() {
    let validator = load_schema();

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  test-evidence: \"{bad}\"\nreview-status:\n  review-record: review-record.yaml\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "test-evidence artifact name {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_review_record_artifact_name() {
    let validator = load_schema();

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  test-evidence: test-evidence.yaml\nreview-status:\n  review-record: \"{bad}\"\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "review-record artifact name {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_documentation_review_record_artifact_name() {
    let validator = load_schema();

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  test-evidence: test-evidence.yaml\nreview-status:\n  review-record: review-record.yaml\ndocumentation-status:\n  documentation-review-record: \"{bad}\"\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "documentation-review-record artifact name {:?} should be rejected",
            bad
        );
    }
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "behavior-coverage:\n  test-evidence: test-evidence.yaml\nreview-status:\n  review-record: review-record.yaml\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_behavior_coverage() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "behavior-coverage:\n  test-evidence: test-evidence.yaml\n  extra: data\nreview-status:\n  review-record: review-record.yaml\ndocumentation-status:\n  documentation-review-record: doc-review.yaml\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in behavior-coverage should be rejected"
    );
}
