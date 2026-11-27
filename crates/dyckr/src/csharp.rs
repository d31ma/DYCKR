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



pub fn entity_line(type_declaration: &Type, entity: &Value) -> String {
    let mapping = crate::csharp::entity_mapping(type_declaration, entity);
    format!(
        "if (entity == typeof({})) return BehaviorJSON.Parse({});\n",
        type_declaration.name,
        crate::csharp::quote(&crate::sorted_json::SortedJson::text(&mapping))
    )
}

pub fn entity_mapping(type_declaration: &Type, entity: &Value) -> Value {
    let mut mapping = serde_json::json!({
        "entity": type_declaration.name,
        "adapter": entity["adapter"],
        "idField": entity["id"],
        "table": type_declaration.table
    });
    crate::csharp::add_partition(entity, &mut mapping);
    mapping
}

fn add_partition(entity: &Value, mapping: &mut Value) {
    if let Some(partition) = entity.get("partition") {
        mapping["partition"] = partition.clone();
    }
}
