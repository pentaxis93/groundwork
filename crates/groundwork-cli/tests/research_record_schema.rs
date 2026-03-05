use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/research-record.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-research-record.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-research-record.yaml"
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
fn valid_research_record() {
    let validator = load_schema();
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_research_record_topic_with_spaces() {
    let validator = load_schema();
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "topic with spaces should be rejected"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_topic() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "findings:\n  - finding one\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing topic should be rejected");
}

#[test]
fn rejects_missing_findings() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing findings should be rejected");
}

#[test]
fn rejects_empty_findings() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings: []\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "empty findings should be rejected");
}

#[test]
fn rejects_missing_sources() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\ndate: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing sources should be rejected");
}

#[test]
fn rejects_missing_date() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\n",
    );
    assert!(!validator.is_valid(&instance), "missing date should be rejected");
}

// ── Pattern rejections ──────────────────────────────────────────

#[test]
fn rejects_invalid_topic_pattern() {
    let validator = load_schema();

    for bad in ["", "Has Spaces", "UPPER", "123-start", "under_score"] {
        let yaml = format!(
            "topic: \"{bad}\"\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "topic {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_date_pattern() {
    let validator = load_schema();

    for bad in ["not-a-date", "2026/03/05", "03-05-2026"] {
        let yaml = format!(
            "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\ndate: \"{bad}\"\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "date {:?} should be rejected",
            bad
        );
    }
}

// ── Source object validation ────────────────────────────────────

#[test]
fn rejects_source_missing_url() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - title: A source\ndate: \"2026-01-01\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "source missing url should be rejected"
    );
}

#[test]
fn accepts_source_without_title() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\n",
    );
    assert!(
        validator.is_valid(&instance),
        "source without optional title should be accepted"
    );
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\ndate: \"2026-01-01\"\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_source() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "topic: my-topic\nfindings:\n  - finding one\nsources:\n  - url: https://example.com\n    title: A source\n    rating: 5\ndate: \"2026-01-01\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in source should be rejected"
    );
}
