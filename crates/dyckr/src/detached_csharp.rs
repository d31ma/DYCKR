use crate::behavior_module::{Method, Type};
use crate::sorted_json::SortedJson;
use serde_json::{Map, Value};


pub fn quote(value: &str) -> String {
        Value::String(value.to_owned()).to_string()
    }

pub fn named_line(parameter: &str, name: &str, value: &Value) -> String {
        format!(
            "if ({parameter}.Name == {}) return BehaviorJSON.Parse({});\n",
            crate::detached_csharp::quote(name),
            crate::detached_csharp::quote(&SortedJson::text(value))
        )
    }

pub fn registered(type_declaration: &Type) -> Map<String, Value> {
        type_declaration
            .methods
            .iter()
            .filter(|method| crate::detached_csharp::invoked(method))
            .map(|method| (method.name.clone(), Value::Object(method.metadata.clone())))
            .collect()
    }

pub fn invoked(method: &Method) -> bool {
        let written = crate::detached_csharp_consts::WRITTEN_BODIES
            .iter()
            .any(|key| method.metadata.contains_key(*key));
        [method.metadata.is_empty(), written] == [false, false]
    }
