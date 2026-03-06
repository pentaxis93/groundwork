mod common;

const SCHEMA_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../schemas/artifact-frontmatter.schema.json");
const SPEC_FIXTURE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/spec-artifact-frontmatter.yaml");
const VERIFICATION_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/verification-artifact-frontmatter.yaml"
);


// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_spec_artifact_frontmatter() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(SPEC_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(validator.is_valid(&instance), "spec fixture should be valid");
}

#[test]
fn valid_verification_artifact_frontmatter() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(VERIFICATION_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        validator.is_valid(&instance),
        "verification fixture should be valid"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_schema() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "freshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(!validator.is_valid(&instance), "missing schema should be rejected");
}

#[test]
fn rejects_missing_freshness() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing freshness should be rejected"
    );
}

#[test]
fn rejects_missing_produced_by() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing produced_by should be rejected"
    );
}

#[test]
fn rejects_missing_produced_at() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing produced_at should be rejected"
    );
}

// ── Enum rejections ─────────────────────────────────────────────

#[test]
fn rejects_unknown_freshness() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: expired\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "unknown freshness value should be rejected"
    );
}

#[test]
fn rejects_unknown_approval() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\napproval: maybe\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "unknown approval value should be rejected"
    );
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_dependency_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\ndepends_on:\n  - artifact: behavior-contract\n    optional: true\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in dependency entry should be rejected"
    );
}

// ── String pattern rejections ───────────────────────────────────

#[test]
fn rejects_invalid_produced_by() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "../traversal", "UPPER", "123-start"] {
        let yaml = format!(
            "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: \"{bad}\"\nproduced_at: \"2026-03-05T14:30:00Z\"\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "produced_by {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_absolute_schema_path() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: /etc/shadow\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
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
        "schema: ../../../etc/passwd\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "traversal schema path should be rejected"
    );
}

#[test]
fn rejects_invalid_produced_at() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in [
        "2026-03-05",                     // date only, no time
        "2026-03-05T14:30:00",            // no timezone
        "not-a-date",                     // garbage
        "2026-03-05 14:30:00Z",           // space instead of T
    ] {
        let yaml = format!(
            "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"{bad}\"\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "produced_at {:?} should be rejected",
            bad
        );
    }
}

// ── Optional field acceptance ───────────────────────────────────

#[test]
fn valid_without_depends_on() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\n",
    );
    assert!(
        validator.is_valid(&instance),
        "omitting depends_on should be valid"
    );
}

#[test]
fn valid_with_empty_depends_on() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\ndepends_on: []\n",
    );
    assert!(
        validator.is_valid(&instance),
        "empty depends_on should be valid"
    );
}

#[test]
fn valid_without_approval() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\ndepends_on:\n  - artifact: behavior-contract\n",
    );
    assert!(
        validator.is_valid(&instance),
        "omitting approval should be valid"
    );
}

// ── Dependency pattern rejections ───────────────────────────────

#[test]
fn rejects_invalid_dependency_artifact_name() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "../traversal", "UPPER", "123-start"] {
        let yaml = format!(
            "schema: schemas/x.schema.json\nfreshness: fresh\nproduced_by: ground\nproduced_at: \"2026-03-05T14:30:00Z\"\ndepends_on:\n  - artifact: \"{bad}\"\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "dependency artifact name {:?} should be rejected",
            bad
        );
    }
}
