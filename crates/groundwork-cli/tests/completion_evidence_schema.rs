mod common;

const SCHEMA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/completion-evidence.schema.json"
);
const VALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/valid-completion-evidence.yaml"
);
const INVALID_FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/artifacts/invalid-completion-evidence.yaml"
);

// ── Valid fixtures ──────────────────────────────────────────────

#[test]
fn valid_completion_evidence() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(VALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        validator.is_valid(&instance),
        "valid fixture should be accepted"
    );
}

// ── Invalid fixture ─────────────────────────────────────────────

#[test]
fn invalid_completion_evidence_bad_status_enum() {
    let validator = common::load_schema(SCHEMA_PATH);
    let text = std::fs::read_to_string(INVALID_FIXTURE).expect("read fixture");
    let instance = common::yaml_to_json(&text);
    assert!(
        !validator.is_valid(&instance),
        "status 'passed' should be rejected (not in enum)"
    );
}

// ── Required field rejections ───────────────────────────────────

#[test]
fn rejects_missing_behavior_coverage() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "review-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing behavior-coverage should be rejected"
    );
}

#[test]
fn rejects_missing_review_artifact() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: test\n    status: pass\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing review-artifact should be rejected"
    );
}

#[test]
fn rejects_missing_documentation_artifact() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: test\n    status: pass\nreview-artifact: review-record\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "missing documentation-artifact should be rejected"
    );
}

// ── Array constraint rejections ─────────────────────────────────

#[test]
fn rejects_empty_behavior_coverage_array() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage: []\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "empty behavior-coverage array should be rejected"
    );
}

// ── Behavior entry required field rejections ────────────────────

#[test]
fn rejects_behavior_entry_missing_behavior() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - status: pass\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "behavior entry missing behavior field should be rejected"
    );
}

#[test]
fn rejects_behavior_entry_missing_status() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: test scenario\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "behavior entry missing status field should be rejected"
    );
}

// ── Enum rejections ─────────────────────────────────────────────

#[test]
fn rejects_invalid_status_values() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["passed", "failed", "missing", "PASS", "success"] {
        let yaml = format!(
            "behavior-coverage:\n  - behavior: test\n    status: {bad}\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "status {:?} should be rejected",
            bad
        );
    }
}

// ── Pattern rejections ──────────────────────────────────────────

#[test]
fn rejects_invalid_review_artifact_pattern() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  - behavior: test\n    status: pass\nreview-artifact: \"{bad}\"\ndocumentation-artifact: doc-review\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "review-artifact {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_documentation_artifact_pattern() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  - behavior: test\n    status: pass\nreview-artifact: review-record\ndocumentation-artifact: \"{bad}\"\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "documentation-artifact {:?} should be rejected",
            bad
        );
    }
}

#[test]
fn rejects_invalid_evidence_artifact_pattern() {
    let validator = common::load_schema(SCHEMA_PATH);

    for bad in ["", "Has Spaces", "UPPER", "123-start"] {
        let yaml = format!(
            "behavior-coverage:\n  - behavior: test\n    status: pass\n    evidence-artifact: \"{bad}\"\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n"
        );
        let instance = common::yaml_to_json(&yaml);
        assert!(
            !validator.is_valid(&instance),
            "evidence-artifact {:?} should be rejected",
            bad
        );
    }
}

// ── additionalProperties rejections ─────────────────────────────

#[test]
fn rejects_extra_field_at_top_level() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: test\n    status: pass\nreview-artifact: review-record\ndocumentation-artifact: doc-review\nversion: \"1.0\"\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field at top level should be rejected"
    );
}

#[test]
fn rejects_extra_field_in_behavior_entry() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: test\n    status: pass\n    priority: high\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "extra field in behavior entry should be rejected"
    );
}

// ── Optional field acceptance ───────────────────────────────────

#[test]
fn valid_without_evidence_artifact() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: untested scenario\n    status: gap\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        validator.is_valid(&instance),
        "gap entry without evidence-artifact should be accepted"
    );
}

#[test]
fn rejects_pass_without_evidence_artifact() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: tested scenario\n    status: pass\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "pass entry without evidence-artifact should be rejected"
    );
}

#[test]
fn rejects_fail_without_evidence_artifact() {
    let validator = common::load_schema(SCHEMA_PATH);
    let instance = common::yaml_to_json(
        "behavior-coverage:\n  - behavior: tested failing scenario\n    status: fail\nreview-artifact: review-record\ndocumentation-artifact: doc-review\n",
    );
    assert!(
        !validator.is_valid(&instance),
        "fail entry without evidence-artifact should be rejected"
    );
}
