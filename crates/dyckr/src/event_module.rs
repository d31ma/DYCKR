use crate::event_method::EventMethod;
use crate::event_type::EventType;

#[derive(Clone, Debug)]
pub struct EventModule {
    pub source: String,
    pub types: Vec<EventType>,
    pub methods: Vec<EventMethod>,
    pub topics: Vec<String>,
}
