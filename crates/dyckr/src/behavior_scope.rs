use crate::behavior_input::Input;
use crate::behavior_marker::Marker;
use crate::failure::Failure;
use serde_json::{Map, Value};

pub fn scope(input: &Input<'_>, marker: &Marker) -> Result<Value, Failure> {
    let options = object(input, marker, &["value"])?;
    let scope = options
        .get("value")
        .and_then(Value::as_str)
        .ok_or_else(|| input.failure(marker.span.start, "Lifetime requires a literal value"))?;
    if !crate::native_application_lifetime::NativeApplicationLifetime::supported(input.path, scope)
    {
        return Err(
            crate::native_application_lifetime::NativeApplicationLifetime::unsupported(
                input.path,
                marker.span.start,
                scope,
            )
            .unwrap_or_else(|| {
                input.failure(
                    marker.span.start,
                    crate::native_application_lifetime::NativeApplicationLifetime::advice(scope),
                )
            }),
        );
    }
    Ok(Value::String(scope.to_owned()))
}

pub fn object(
    input: &Input<'_>,
    marker: &Marker,
    keys: &[&str],
) -> Result<Map<String, Value>, Failure> {
    let value: Value = crate::behavior_literal::parse(&marker.arguments).map_err(|_| {
        input.failure(
            marker.span.start,
            "Behavior configuration must be a strict JSON object, never an executable expression",
        )
    })?;
    let object = value.as_object().ok_or_else(|| {
        input.failure(
            marker.span.start,
            "Behavior configuration must be a JSON object",
        )
    })?;
    if object.len() != keys.len() || object.keys().any(|key| !keys.contains(&key.as_str())) {
        return Err(input.failure(
            marker.span.start,
            format!(
                "{} requires exactly these keys: {}",
                marker.name,
                keys.join(", ")
            ),
        ));
    }
    Ok(object.clone())
}
