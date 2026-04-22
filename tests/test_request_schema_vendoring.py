import json
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REQUEST_SCHEMA_PATH = ROOT / "schemas" / "request.schema.json"
SCHEMAS_README_PATH = ROOT / "schemas" / "README.md"
VALID_REQUEST_FIXTURE_PATH = ROOT / "tests" / "fixtures" / "artifacts" / "valid-request.json"
INVALID_REQUEST_FIXTURE_PATH = ROOT / "tests" / "fixtures" / "artifacts" / "invalid-request.json"


def validate_request_instance(schema: dict, instance: dict) -> None:
    if schema["type"] != "object":
        raise AssertionError("request schema must validate an object")
    if not isinstance(instance, dict):
        raise ValueError("instance must be an object")

    properties = schema["properties"]

    for field in schema["required"]:
        if field not in instance:
            raise ValueError(f"missing required field: {field}")

    if schema["additionalProperties"] is False:
        unknown_fields = set(instance) - set(properties)
        if unknown_fields:
            raise ValueError(f"unexpected fields: {sorted(unknown_fields)!r}")

    for field, value in instance.items():
        field_schema = properties[field]
        if field_schema["type"] == "string":
            if not isinstance(value, str):
                raise ValueError(f"{field} must be a string")
            if "minLength" in field_schema and len(value) < field_schema["minLength"]:
                raise ValueError(f"{field} must be at least {field_schema['minLength']} characters")


class RequestSchemaVendoringTests(unittest.TestCase):
    def test_request_schema_declares_canonical_release_provenance(self) -> None:
        schema = json.loads(REQUEST_SCHEMA_PATH.read_text())

        self.assertNotIn("$id", schema)
        self.assertEqual(
            schema["x-tesserine-canonical"],
            {
                "version": "1.0.0",
                "schema_url": (
                    "https://raw.githubusercontent.com/tesserine/commons/"
                    "v0.1.1/schemas/request/v1/request.schema.json"
                ),
                "prose_url": (
                    "https://raw.githubusercontent.com/tesserine/commons/"
                    "v0.1.1/REQUEST.md"
                ),
            },
        )

    def test_schemas_readme_documents_request_vendoring_discipline(self) -> None:
        readme = SCHEMAS_README_PATH.read_text()

        self.assertIn("methodology-private", readme)
        self.assertIn("request.schema.json", readme)
        self.assertIn("tesserine/commons", readme)
        self.assertIn("runtime consumers still read schemas from groundwork", readme)
        self.assertIn("immutable release-tag or commit-SHA URL", readme)
        self.assertIn("full semver", readme)

    def test_valid_request_fixture_still_matches_vendored_schema_contract(self) -> None:
        schema = json.loads(REQUEST_SCHEMA_PATH.read_text())
        fixture = json.loads(VALID_REQUEST_FIXTURE_PATH.read_text())

        validate_request_instance(schema, fixture)

    def test_invalid_request_fixture_still_fails_vendored_schema_contract(self) -> None:
        schema = json.loads(REQUEST_SCHEMA_PATH.read_text())
        fixture = json.loads(INVALID_REQUEST_FIXTURE_PATH.read_text())

        with self.assertRaisesRegex(ValueError, "missing required field: source"):
            validate_request_instance(schema, fixture)


if __name__ == "__main__":
    unittest.main()
