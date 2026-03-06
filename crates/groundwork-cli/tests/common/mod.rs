use jsonschema::Validator;
use serde_json::Value;

pub fn load_schema(path: &str) -> Validator {
    let text = std::fs::read_to_string(path).expect("read schema");
    let value: Value = serde_json::from_str(&text).expect("parse schema JSON");
    Validator::new(&value).expect("compile schema")
}

pub fn yaml_to_json(yaml: &str) -> Value {
    serde_yml::from_str(yaml).expect("parse YAML")
}
