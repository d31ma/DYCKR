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
use crate::behavior_module::Method;


pub fn json_arguments(method: &Method) -> String {
    let mut arguments = Vec::new();
    for parameter in &method.parameters {
        arguments.push(format!("BehaviorJSON.From({})", parameter.name));
    }
    arguments.join(", ")
}
use crate::behavior_unit::Unit;


pub fn joined<T>(items: impl IntoIterator<Item = T>, spell: impl Fn(T) -> String) -> String {
    let mut text = String::new();
    for item in items {
        text.push_str(&spell(item));
    }
    text
}

pub fn test_context(unit: &Unit) -> String {
    format!(
        "Behaviors.create(typeof({}), {}, new BehaviorSignal(), 10000, new string[] {{{}}}, \
         new System.Collections.Generic.Dictionary<string, string> {{ {} }})",
        unit.configuration,
        crate::csharp::quote(&unit.partition),
        profile_list(unit),
        property_entries(unit),
    )
}

pub fn case_names(calls: &[(String, String)]) -> String {
    let mut names = Vec::new();
    for (name, _) in calls {
        names.push(crate::csharp::quote(name));
    }
    names.join(", ")
}

pub fn profile_list(unit: &Unit) -> String {
    let Some(values) = unit.profiles.as_array() else {
        return String::new();
    };
    let mut profiles = Vec::new();
    for value in values {
        if let Some(profile) = value.as_str() {
            profiles.push(crate::csharp::quote(profile));
        }
    }
    profiles.join(", ")
}

pub fn property_entries(unit: &Unit) -> String {
    let Some(entries) = unit.properties.as_object() else {
        return String::new();
    };
    crate::csharp::joined(crate::sorted_json::SortedJson::entries(entries), |(key, value)| {
        let name = key.replace('.', "_").to_uppercase();
        let text = value
            .as_str()
            .map_or_else(|| crate::sorted_json::SortedJson::text(value), str::to_owned);
        format!("[{}] = {}, ", crate::csharp::quote(&name), crate::csharp::quote(&text))
    })
}


pub fn is_json(value: &str) -> bool {
    value.trim() == "BehaviorJSON"
}

pub fn is_task_json(value: &str) -> bool {
    let without_spaces = value.split_whitespace().collect::<String>();
    matches!(
        without_spaces.as_str(),
        "Task<BehaviorJSON>" | "System.Threading.Tasks.Task<BehaviorJSON>"
    )
}

pub fn returns_json_or_task_json(method: &Method) -> bool {
    crate::csharp::is_json(&method.return_type) || (method.asynchronous && crate::csharp::is_task_json(&method.return_type))
}


pub fn is_validation_only(method: &Method) -> bool {
    method.metadata.len() == 1 && method.metadata.contains_key("valid")
}

pub fn is_declared_exchange(method: &Method) -> bool {
    let Some(exchange) = method.metadata.get("httpExchange") else {
        return false;
    };
    exchange.get("generated").is_some()
}

pub fn uses_invocation(method: &Method) -> bool {
    method.metadata.contains_key("cacheable")
        || method.metadata.contains_key("httpExchange")
        || method.asynchronous
        || method.metadata.contains_key("cacheEvict")
        || method.metadata.contains_key("cachePut")
}
