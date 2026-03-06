mod common;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/groundwork-frontmatter.schema.json");
const CORE_FIXTURE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/core-skill-frontmatter.yaml");
const WRAPPER_FIXTURE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/wrapper-skill-frontmatter.yaml");


// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_core_skill_frontmatter() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(CORE_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "core fixture should be valid");
}

#[test]
fn valid_wrapper_skill_frontmatter() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(WRAPPER_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "wrapper fixture should be valid");
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_stage() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json("groundwork:\n  requires: []\n  produces: []\n");
    assert!(!validator.is_valid(&instance), "missing stage should be rejected");
}

#[test]
fn rejects_unknown_stage() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: planning\n  requires: []\n  produces: []\n",
    );
    assert!(!validator.is_valid(&instance), "unknown stage should be rejected");
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_in_groundwork() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces: []\n  version: '1.0'\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in groundwork object should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_artifact_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - artifact: verified-constraints\n      schema: schemas/verified-constraints.schema.json\n      optional: true\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in artifact entry should be rejected"
    );
}

// ── Required fields in artifact entries ─────────────────────────

#[test]
fn rejects_missing_artifact_in_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - schema: schemas/foo.schema.json\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "entry missing artifact should be rejected"
    );
}

#[test]
fn rejects_missing_schema_in_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - artifact: foo\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "entry missing schema should be rejected"
    );
}

// ── String pattern rejections ───────────────────────────────────

#[test]
fn rejects_invalid_artifact_name() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "../traversal", "UPPER", "123-start"] {
        let yaml = format!(
            "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - artifact: \"{bad}\"\n      schema: schemas/x.schema.json\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "artifact name {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_absolute_schema_path() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - artifact: foo\n      schema: /etc/shadow\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "absolute schema path should be rejected"
    );
}

#[test]
fn rejects_traversal_schema_path() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "groundwork:\n  stage: specification\n  requires: []\n  produces:\n    - artifact: foo\n      schema: ../../../etc/passwd\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "traversal schema path should be rejected"
    );
}
