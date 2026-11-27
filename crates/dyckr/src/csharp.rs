use crate::behavior_module::Type;


pub fn configuration_line(type_declaration: &Type) -> String {
    format!(
        "if (configuration == typeof({})) return {}.configure(execution);\n",
        type_declaration.name, type_declaration.name
    )
}
use serde_json::Value;

pub fn quote(value: &str) -> String {
    Value::String(value.to_owned()).to_string()
}

