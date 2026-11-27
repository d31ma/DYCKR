use crate::event_type::EventType;
use crate::event_method::EventMethod;

#[derive(Clone, Debug)]
pub struct EventModule {
    pub source: String,
    pub types: Vec<EventType>,
    pub methods: Vec<EventMethod>,
    pub topics: Vec<String>,
}
