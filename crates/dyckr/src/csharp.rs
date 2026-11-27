use crate::behavior_module::{Edit, Method, Module, Parameter, Type};


pub fn configuration_line(type_declaration: &Type) -> String {
    format!(
        "if (configuration == typeof({})) return {}.configure(execution);\n",
        type_declaration.name, type_declaration.name
    )
}
use serde_json::{Map, Value};

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


pub fn json_arguments(method: &Method) -> String {
    let mut arguments = Vec::new();
    for parameter in &method.parameters {
        arguments.push(format!("BehaviorJSON.From({})", parameter.name));
    }
    arguments.join(", ")
}


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
        names.push(crate::csharp::quote(&name));
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


pub fn policy_line(type_declaration: &Type) -> String {
    let policies = crate::sorted_json::SortedJson::text(&Value::Object(type_declaration.policies.clone()));
    format!(
        "if (type == typeof({})) return BehaviorJSON.Parse({});\n",
        type_declaration.name,
        crate::csharp::quote(&policies)
    )
}


pub fn parameter_at<'a>(
    method: &'a Method,
    policy: &Value,
    annotation_name: &str,
) -> Result<&'a Parameter, String> {
    let Some(position) = policy["argument"].as_u64() else {
        return Err(format!("Missing {annotation_name} argument"));
    };
    let out_of_range = format!("{annotation_name} argument is out of range");
    let Ok(index) = usize::try_from(position) else {
        return Err(out_of_range);
    };
    method.parameters.get(index).ok_or(out_of_range)
}


pub fn pageable_line(method: &Method, policy: &Value) -> Result<String, String> {
    let parameter = crate::csharp::parameter_at(method, policy, "Pageable")?;
    if !crate::csharp::is_json(&parameter.type_source) {
        return Err("C# Pageable parameters must have type BehaviorJSON".to_owned());
    }
    Ok(format!(
        "{} = BehaviorPage.Normalize({}, BehaviorJSON.Parse({}));\n",
        parameter.name,
        parameter.name,
        crate::csharp::quote(&crate::sorted_json::SortedJson::text(policy))
    ))
}


pub fn validation_prefix(method: &Method) -> Result<String, String> {
    let Some(policy) = method.metadata.get("valid") else {
        return Ok(String::new());
    };
    let parameter = crate::csharp::parameter_at(method, policy, "Check")?;
    if !crate::csharp::is_json(&parameter.type_source) {
        return Err("C# Check parameters require BehaviorJSON".to_owned());
    }
    Ok(format!(
        "{} = Behaviors.CheckSchema(BehaviorJSON.Parse({}), {});\n",
        parameter.name,
        crate::csharp::quote(&crate::sorted_json::SortedJson::text(&policy["schema"])),
        parameter.name
    ))
}


pub fn gateway_body(method: &Method, policy: &Value) -> String {
    let key = policy["key"].as_str().unwrap_or_default();
    let mut names = Vec::new();
    for parameter in &method.parameters {
        names.push(parameter.name.as_str());
    }
    format!(
        "\nreturn Behaviors.GatewayResult(YonGateway.Call({}, Behaviors.GatewayArguments(new BehaviorJSON[] {{ {} }})));\n",
        crate::csharp::quote(key),
        names.join(", ")
    )
}

pub fn mapping_body(method: &Method, policy: &Value) -> Result<String, String> {
    let result = policy.get("result").and_then(Value::as_str);
    if result.is_none() && !crate::csharp::returns_json_or_task_json(method) {
        return Err(
            "C# mapping methods require BehaviorJSON or async Task<BehaviorJSON>".to_owned(),
        );
    }
    let (argument, _) = crate::mapper::plan(policy)?;
    let value = method
        .parameters
        .get(argument)
        .ok_or("Convert reads an argument the method does not declare")?;
    let plan = crate::csharp::quote(&crate::sorted_json::SortedJson::text(policy));
    let Some(result) = result else {
        return Ok(format!(
            "\nreturn BehaviorContext.MapValue(BehaviorJSON.Parse({plan}), BehaviorJSON.From({}));\n",
            value.name
        ));
    };
    if policy["realm"] == "tac" {
        return Ok(format!(
            "\nreturn TacModels.Read<{result}>(Tac.ParseValue(BehaviorContext.MapValue(BehaviorJSON.Parse({plan}), BehaviorJSON.Parse(Tac.EncodeValue(TacModels.Plain({})))).ToString()));\n",
            value.name
        ));
    }
    Ok(format!(
        "\nreturn System.Text.Json.JsonSerializer.Deserialize<{result}>(BehaviorContext.MapValue(BehaviorJSON.Parse({plan}), BehaviorJSON.Parse(System.Text.Json.JsonSerializer.Serialize({}))).ToString(), YonModels.Options)!;\n",
        value.name
    ))
}

pub fn query_body(method: &Method, query: &Value) -> Result<String, String> {
    if !method.asynchronous || !crate::csharp::is_task_json(&method.return_type) {
        return Err("C# query methods require async Task<BehaviorJSON>".to_owned());
    }
    let arguments = crate::csharp::json_arguments(method);
    Ok(format!(
        "\nreturn await this.behaviors.Query(BehaviorJSON.Parse({}), new BehaviorJSON[] {{ {arguments} }});\n",
        crate::csharp::quote(&crate::sorted_json::SortedJson::text(query))
    ))
}


pub fn written_body(method: &Method) -> Result<Option<String>, String> {
    if let Some(policy) = method.metadata.get("gateway") {
        return Ok(Some(crate::csharp::gateway_body(method, policy)));
    }
    if let Some(query) = method.metadata.get("query") {
        return crate::csharp::query_body(method, query).map(Some);
    }
    if let Some(policy) = method.metadata.get("mapping") {
        return crate::csharp::mapping_body(method, policy).map(Some);
    }
    Ok(None)
}


pub fn validation_only_body(method: &Method, prefix: &str, body: &str) -> Result<String, String> {
    if !crate::csharp::returns_json_or_task_json(method) {
        return Err("C# Check methods require BehaviorJSON or async Task<BehaviorJSON>".to_owned());
    }
    Ok(format!("\n{prefix}{body}\n"))
}

pub fn invocation_body(
    type_declaration: &Type,
    method: &Method,
    prefix: &str,
    body: &str,
) -> Result<String, String> {
    if !method.asynchronous || !crate::csharp::is_task_json(&method.return_type) {
        return Err("C# asynchronous behavior methods require async Task<BehaviorJSON>".to_owned());
    }
    let arguments = crate::csharp::json_arguments(method);
    Ok(format!(
        "\n{prefix}return await this.behaviors.Invoke({}, {}, new BehaviorJSON[] {{ {arguments} }}, async () => {{ {body} }});\n",
        crate::csharp::quote(&type_declaration.name),
        crate::csharp::quote(&method.name)
    ))
}


pub fn constructor_parameters(type_declaration: &Type, fields: &[String]) -> Result<String, String> {
    let mut parameters = Vec::new();
    for name in fields {
        let Some(field) = type_declaration
            .fields
            .iter()
            .find(|field| field.name == *name)
        else {
            return Err(format!(
                "Required constructor field '{name}' has no explicit C# type"
            ));
        };
        parameters.push(format!("{} {name}", field.type_source));
    }
    Ok(parameters.join(", "))
}


pub fn declared_exchange(
    type_declaration: &Type,
    method: &Method,
    prefix: &str,
) -> Result<String, String> {
    if !method.asynchronous || !crate::csharp::is_task_json(&method.return_type) {
        return Err("C# declared WebCall methods require async Task<BehaviorJSON>".to_owned());
    }
    let arguments = crate::csharp::json_arguments(method);
    Ok(format!(
        "\n{prefix}return await this.behaviors.Invoke({}, {}, new BehaviorJSON[] {{ {arguments} }}, () => System.Threading.Tasks.Task.FromResult(BehaviorJSON.Parse(\"null\")));\n",
        crate::csharp::quote(&type_declaration.name),
        crate::csharp::quote(&method.name)
    ))
}


pub fn method_body(
    type_declaration: &Type,
    method: &Method,
    prefix: &str,
    body: &str,
    exchange_body: &dyn Fn(&Type, &Method, &str) -> Option<String>,
) -> Result<String, String> {
    if let Some(body) =
        exchange_body(type_declaration, method, prefix)
    {
        return Ok(body);
    }
    if crate::csharp::is_validation_only(method) {
        return crate::csharp::validation_only_body(method, prefix, body);
    }
    if crate::csharp::is_declared_exchange(method) {
        return crate::csharp::declared_exchange(type_declaration, method, prefix);
    }
    if crate::csharp::uses_invocation(method) {
        return crate::csharp::invocation_body(type_declaration, method, prefix, body);
    }
    if method.return_type.trim() == "void" {
        return Err("C# Pageable methods must return a value".to_owned());
    }
    Ok(format!(
        "\n{prefix}return this.behaviors.Paginate(() => {{ {body} }});\n"
    ))
}


pub fn constructor_edit(type_declaration: &Type, fields: &[String]) -> Result<Edit, String> {
    let parameters = crate::csharp::constructor_parameters(type_declaration, fields)?;
    let mut assignments = Vec::new();
    for name in fields {
        assignments.push(format!("this.{name} = {name};"));
    }
    let assignments = assignments.join(" ");
    let declarations = crate::polyglot::declared_fields(type_declaration, |field| {
        format!("\nreadonly {} {};", field.type_source, field.name)
    });
    Ok(Edit {
        range: type_declaration.body.start..type_declaration.body.start,
        replacement: format!(
            "{declarations}\npublic {}({parameters}) {{ {assignments} }}\n",
            type_declaration.name
        ),
    })
}

pub fn public_members(type_declaration: &Type) -> impl Iterator<Item = Edit> + '_ {
    type_declaration
        .methods
        .iter()
        .filter(|method| method.declared)
        .map(|method| Edit {
            range: method.signature.start..method.signature.start,
            replacement: "public ".to_owned(),
        })
}

pub fn lower_methods(
    module: &Module,
    type_declaration: &Type,
    edits: &mut Vec<Edit>,
    exchange_body: &dyn Fn(&Type, &Method, &str) -> Option<String>,
) -> Result<Map<String, Value>, String> {
    let mut methods = Map::new();
    edits.extend(crate::csharp::public_members(type_declaration));
    for method in &type_declaration.methods {
        if method.metadata.is_empty() {
            continue;
        }
        if let Some(replacement) = crate::csharp::written_body(method)? {
            edits.push(Edit {
                range: method.body.clone(),
                replacement: crate::polyglot::block(method, &replacement),
            });
            continue;
        }
        let mut prefix = crate::csharp::validation_prefix(method)?;
        if let Some(policy) = method.metadata.get("pageable") {
            prefix.push_str(&crate::csharp::pageable_line(method, policy)?);
        }
        let body = &module.source[method.body.clone()];
        let replacement = crate::csharp::method_body(type_declaration, method, &prefix, body, exchange_body)?;
        edits.push(Edit {
            range: method.body.clone(),
            replacement: crate::polyglot::block(method, &replacement),
        });
        methods.insert(method.name.clone(), Value::Object(method.metadata.clone()));
    }
    Ok(methods)
}

pub struct CsharpErrorBody;

impl CsharpErrorBody {
    pub const READER: &'static str = "TryReadBody";
    pub const SOURCE: &'static str = "public sealed partial class BehaviorError {\n    public bool TryReadBody<T>(out T model) => TacModels.ReadErrorBody(Body?.ToString(), out model);\n}\n";

    pub fn reader(module: &Module) -> &'static str {
        let wanted = [
            module.environment == "tac",
            module.source.contains(Self::READER),
        ] == [true, true];
        ["", Self::SOURCE][usize::from(wanted)]
    }
}

use crate::behavior_unit::Unit;


pub fn test_members(unit: &Unit, environment: &str, calls: &[(String, String)]) -> String {
    let (report, answer) = if environment == "tac" {
        ("string", "report.ToString()")
    } else {
        (
            "object",
            "System.Text.Json.Nodes.JsonNode.Parse(report.ToString())",
        )
    };
    let names = crate::csharp::case_names(&calls);
    let dispatch = crate::csharp::joined(calls.iter().cloned(), |(name, call)| {
        format!("case {}: RunCaseBody(() => {call}); return;", crate::csharp::quote(&name))
    });
    let context = crate::csharp::test_context(unit);
    let cleared_fakes = crate::csharp::joined(&unit.mocks, |name| format!("this.{name} = default;\n"));
    format!(
        "\npublic BehaviorContext behaviors;\n\
         public void startBehaviors() {{ stopBehaviors(); this.behaviors = {context}; }}\n\
         public void stopBehaviors() {{ if (this.behaviors != null) {{ this.behaviors.Dispose(); this.behaviors = null; }} }}\n\
         static void RunCaseBody(System.Action body) => body();\n\
         static void RunCaseBody(System.Func<System.Threading.Tasks.Task> body) => body().GetAwaiter().GetResult();\n\
         void runCase(string name) {{ switch (name) {{ {dispatch} default: return; }} }}\n\
         public {report} POST() {{\n\
         var cases = new System.Collections.Generic.List<BehaviorJSON>();\n\
         foreach (var name in new string[] {{{names}}}) {{\n\
         {cleared_fakes}\
         startBehaviors();\n\
         BehaviorCalls.TakeJson();\n\
         try {{ runCase(name); cases.Add(BehaviorJSON.Parse(\"{{\\\"name\\\":\" + Quoted(name) + \",\\\"passed\\\":true,\\\"called\\\":\" + BehaviorCalls.TakeJson() + \"}}\")); }}\n\
         catch (System.Exception error) {{ cases.Add(BehaviorJSON.Parse(\"{{\\\"name\\\":\" + Quoted(name) + \",\\\"passed\\\":false,\\\"error\\\":\" + Quoted(error.Message ?? \"failed\") + \",\\\"called\\\":\" + BehaviorCalls.TakeJson() + \"}}\")); }}\n\
         finally {{ stopBehaviors(); }}\n\
         }}\n\
         var report = BehaviorJSON.Object(new System.Collections.Generic.Dictionary<string, BehaviorJSON> {{\n\
         [\"unit\"] = BehaviorJSON.From({}), [\"cases\"] = BehaviorJSON.Array(cases.ToArray()),\n\
         }});\n\
         return {answer};\n\
         }}\n\
         static string Quoted(string value) => BehaviorJSON.Quote(value);\n",
        crate::csharp::quote(&unit.name),
    )
}


pub fn lower(module: &Module, runtime: &dyn Fn(&str, &str, &str, &str, &str, &str) -> String, test_block: &dyn Fn(&Module, &Type, &Unit) -> String, unit_helper: &dyn Fn(&Module) -> &'static str, exchange_body: &dyn Fn(&Type, &Method, &str) -> Option<String>) -> Result<String, String> {
    let mut edits = Vec::new();
    let mut registrations = Map::new();
    let mut configurations = String::new();
    let mut entities = String::new();
    let mut policies = String::new();
    for type_declaration in &module.types {
        edits.extend(crate::polyglot::concrete_edit(module, type_declaration));
        policies.push_str(&crate::csharp::policy_line(type_declaration));
        if type_declaration.configuration {
            configurations.push_str(&crate::csharp::configuration_line(type_declaration));
        }
        if let Some(entity) = &type_declaration.entity {
            entities.push_str(&crate::csharp::entity_line(type_declaration, entity));
        }
        if let Some(fields) = &type_declaration.constructor {
            edits.push(crate::csharp::constructor_edit(type_declaration, fields)?);
        }
        if let Some(unit) = crate::polyglot_unit::polyglot_unit(type_declaration) {
            edits.push(Edit {
                range: type_declaration.body.start..type_declaration.body.start,
                replacement: test_block(module, type_declaration, &unit),
            });
        }
        let methods = crate::csharp::lower_methods(module, type_declaration, &mut edits, exchange_body)?;
        registrations.insert(
            type_declaration.name.clone(),
            serde_json::json!({"cache": type_declaration.cache, "methods": methods}),
        );
    }
    let registry = crate::sorted_json::SortedJson::text(&Value::Object(registrations));
    let mut generated = runtime(
        &module.module_id,
        &module.environment,
        &registry,
        &configurations,
        &entities,
        &policies,
    );
    generated.push_str(crate::csharp::CsharpErrorBody::reader(module));
    generated.push_str(unit_helper(module));
    crate::polyglot::render(module, edits, &generated)
}
pub fn runtime(
    module_id: &str,
    environment: &str,
    registry: &str,
    configurations: &str,
    entities: &str,
    policies: &str,
    sources: &[&str],
) -> String {
    sources.concat()
    .replace("__TAC_BEHAVIOR_MODULE__", &crate::csharp::quote(module_id))
    .replace("__TAC_BEHAVIOR_ENVIRONMENT__", &crate::csharp::quote(environment))
    .replace("__TAC_BEHAVIOR_REGISTRY__", &crate::csharp::quote(registry))
    .replace("__TAC_BEHAVIOR_CONFIGURATIONS__", configurations)
    .replace("__TAC_BEHAVIOR_ENTITIES__", entities)
    .replace("__TAC_BEHAVIOR_TYPES__", policies)
}


