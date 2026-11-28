use super::ValueSource;
use super::testing::Unit;
use serde_json::Value;

pub struct UnitTable {
    pub name: String,
    pub configuration: String,
    pub partition: String,
    pub profiles: Value,
    pub properties: Value,
    pub cases: Value,
    pub fakes: Vec<String>,
}

impl UnitTable {
    pub(super) fn of(unit: &Unit) -> Self {
        Self {
            name: unit.name.clone(),
            configuration: unit.configuration.clone(),
            partition: unit.partition.clone(),
            profiles: unit.profiles.clone(),
            properties: unit.properties.clone(),
            cases: ValueSource::case_table(&unit.cases, &unit.inputs),
            fakes: unit.mocks.clone(),
        }
    }
}
