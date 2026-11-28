use crate::behavior_unit::Unit;

pub fn polyglot_unit(ty: &crate::behavior_module::Type) -> Option<Unit> {
    let declared = ty.policies.get("test")?.as_object()?;
    let configuration = declared.get("configuration")?.as_str()?.to_owned();
    let partition = declared
        .get("partition")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("test")
        .to_owned();
    let profiles = ty
        .policies
        .get("activeProfiles")
        .cloned()
        .unwrap_or_else(|| serde_json::Value::Array(Vec::new()));
    let properties = ty
        .policies
        .get("testProperties")
        .cloned()
        .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
    Some(Unit {
        name: ty.name.clone(),
        configuration,
        partition,
        profiles,
        properties,
        cases: crate::string_list::string_list(&ty.policies, "cases"),
        inputs: crate::value_source::ValueSource::inputs(&ty.policies),
        mocks: crate::string_list::string_list(&ty.policies, "mockBeans"),
    })
}
