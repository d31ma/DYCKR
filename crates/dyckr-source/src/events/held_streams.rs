use super::{EventModule, EventStream, EventType};

pub struct HeldStreams;

impl HeldStreams {
    pub const MOST_STREAMS: usize = 16;

    pub fn holds_only_streams(event_type: &EventType) -> bool {
        [
            event_type.listeners.is_empty(),
            event_type.resources.is_empty(),
            event_type.post_construct.is_none(),
            event_type.pre_destroy.is_none(),
            !event_type.streams.is_empty(),
        ]
        .iter()
        .all(|holds| *holds)
    }

    pub fn is_client(event_type: &EventType, owner: &str) -> bool {
        [
            event_type.name != owner,
            Self::holds_only_streams(event_type),
        ] == [true, true]
    }

    pub fn clients<'a>(module: &'a EventModule, owner: &str) -> Vec<&'a EventType> {
        module
            .types
            .iter()
            .filter(|event_type| Self::is_client(event_type, owner))
            .collect()
    }

    pub fn held<'a>(
        clients: &[&'a EventType],
        field_types: &[String],
    ) -> Vec<(&'a str, &'a EventStream)> {
        clients
            .iter()
            .filter(|client| field_types.contains(&client.name))
            .flat_map(|client| {
                client
                    .streams
                    .iter()
                    .filter(|stream| !stream.topic)
                    .map(|stream| (client.name.as_str(), stream))
            })
            .collect()
    }
}
