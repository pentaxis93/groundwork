use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/behavior-contract.schema.json");
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-behavior-contract.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-behavior-contract.yaml"
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
fn valid_behavior_contract() {
    let validator = load_schema();
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "valid fixture should be accepted");
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_behavior_contract_missing_then() {
    let validator = load_schema();
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = yaml_to_json(&text);
    assert!(!validator.is_valid(&instance), "scenario missing 'then' should be rejected");
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_title() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "scenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing title should be rejected");
}

#[test]
fn rejects_missing_scenarios() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing scenarios should be rejected");
}

#[test]
fn rejects_empty_scenarios() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios: []\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "empty scenarios should be rejected");
}

#[test]
fn rejects_missing_metadata() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\n",
    );
    assert!(!validator.is_valid(&instance), "missing metadata should be rejected");
}

// ── Scenario required field rejections ──────────────────────────

#[test]
fn rejects_scenario_missing_name() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "scenario missing name should be rejected");
}

#[test]
fn rejects_scenario_missing_given() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "scenario missing given should be rejected");
}

#[test]
fn rejects_scenario_missing_when() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(!validator.is_valid(&instance), "scenario missing when should be rejected");
}

// ── Metadata field rejections ───────────────────────────────────

#[test]
fn rejects_missing_produced_by_in_metadata() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  date: \"2026-01-01\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "metadata missing produced_by should be rejected"
    );
}

#[test]
fn rejects_missing_date_in_metadata() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "metadata missing date should be rejected"
    );
}

// ── Pattern rejections ──────────────────────────────────────────

#[test]
fn rejects_invalid_produced_by_pattern() {
    let validator = load_schema();

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: \"{bad}\"\n  date: \"2026-01-01\"\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "produced_by {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_date_pattern() {
    let validator = load_schema();

    for bad in ["not-a-date", "2026/03/05", "03-05-2026"] {
        let yaml = format!(
            "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"{bad}\"\n"
        );
        let instance = yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "date {:?} should be rejected",
            bad
        );
    }
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_scenario() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\n    priority: high\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in scenario should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_metadata() {
    let validator = load_schema();
    let instance = yaml_to_json(
        "title: A contract\nscenarios:\n  - name: s1\n    given: g\n    when: w\n    then: t\nmetadata:\n  produced_by: skill-a\n  date: \"2026-01-01\"\n  version: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in metadata should be rejected"
    );
}
