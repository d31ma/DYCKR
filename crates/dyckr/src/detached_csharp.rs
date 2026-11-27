use crate::behavior_module::{Method, Module, Type};
use crate::planned_type::PlannedType;
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

pub fn configured(runtime: &str) -> String {
        runtime.replacen(
            crate::detached_csharp_consts::CONFIGURE,
            &format!("{}{}", crate::detached_csharp_consts::CONFIGURATIONS_FIELD, crate::detached_csharp_consts::CONFIGURE),
            1,
        )
    }

pub fn policies(module: &Module) -> String {
        module.types.iter().map(crate::detached_csharp::policy_line).collect()
    }

pub fn policy_line(type_declaration: &Type) -> String {
        crate::detached_csharp::named_line(
            "type",
            &type_declaration.name,
            &Value::Object(type_declaration.policies.clone()),
        )
    }

pub fn registry(module: &Module) -> String {
        let registrations: Map<String, Value> = module
            .types
            .iter()
            .map(|type_declaration| {
                (
                    type_declaration.name.clone(),
                    serde_json::json!({"cache": type_declaration.cache, "methods": crate::detached_csharp::registered(type_declaration)}),
                )
            })
            .collect();
        SortedJson::text(&Value::Object(registrations))
    }

pub fn storage(module: &Module) -> &'static str {
        let tac = module.environment == crate::detached_csharp_consts::TAC;
        [crate::detached_csharp_consts::NO_STORAGE, crate::detached_csharp_consts::PAGE_STORAGE][usize::from(tac)]
    }

pub fn mapped_argument(method: &Method) -> Option<usize> {
        let policy = method.metadata.get("mapping")?;
        crate::mapper::plan(policy).ok().map(|(argument, _)| argument)
    }

pub fn types(module: &Module) -> Vec<PlannedType> {
        module
            .types
            .iter()
            .map(|type_declaration| PlannedType {
                name: type_declaration.name.clone(),
                line: crate::detached_csharp::line(module, type_declaration.keyword.start),
                declared: type_declaration.declared,
                takes_context: crate::detached_csharp::takes_context(type_declaration),
            })
            .collect()
    }

pub fn takes_context(type_declaration: &Type) -> bool {
        type_declaration
            .constructor
            .as_ref()
            .is_some_and(|fields| fields.iter().any(|field| field == crate::detached_csharp_consts::CONTEXT))
    }

pub fn line(module: &Module, offset: usize) -> usize {
        module
            .source
            .get(..offset)
            .map_or(0, |before| before.matches('\n').count())
    }
pub struct DetachedCsharp {
    pub lowered: String,
    pub runtime: String,
    pub setups: String,
    pub methods: Vec<crate::planned_method::PlannedMethod>,
    pub types: Vec<crate::planned_type::PlannedType>,
    pub test_context: String,
}

impl DetachedCsharp {

    pub fn with_lowered(self, lowered: String) -> DetachedCsharp {
        DetachedCsharp { lowered, ..self }
    }

    pub fn plain(lowered: String) -> DetachedCsharp {
        DetachedCsharp {
            lowered,
            runtime: String::new(),
            setups: String::new(),
            methods: Vec::new(),
            types: Vec::new(),
            test_context: String::new(),
        }
    }

    pub fn without_page_storage(self) -> DetachedCsharp {
        let setups = self.setups.replace(crate::detached_csharp_consts::PAGE_STORAGE, crate::detached_csharp_consts::NO_STORAGE);
        DetachedCsharp { setups, ..self }
    }
}
