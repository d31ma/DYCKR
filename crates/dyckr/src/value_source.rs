use crate::behavior_input::Input;
use crate::behavior_marker::Marker;
use crate::call_style::CallStyle;
use crate::failure::Failure;
use serde_json::{Map, Value};

pub struct ValueSource;

impl ValueSource {
    pub const NAME: &'static str = "ValueSource";
    pub const DATA_KEY: &'static str = "caseInputs";
    pub const CASE_PREFIXES: [&'static str; 2] = ["test", "Test"];
    pub const MAX_INPUTS: usize = 256;
    pub const NOT_A_CASE: &'static str =
        "ValueSource goes on a test case: a method whose name begins with `test` (`Test` in Go).";
    pub const NOT_A_LIST: &'static str = "ValueSource takes one list of 1 to 256 literal inputs, for example ValueSource([1, 2, 3]).";
    pub const REPEATED: &'static str = "A test case has at most one ValueSource.";
    pub const NOT_SCALAR: &'static str = "In Rust, Java, Kotlin, Groovy, C#, F#, Go, Dart and Swift, ValueSource lists numbers, strings or booleans, for example ValueSource([1, 2, 3]).";

    pub fn record(
        input: &Input<'_>,
        marker: &Marker,
        method: &str,
        data: &mut Map<String, Value>,
    ) -> Result<(), Failure> {
        Self::require_case(input, marker, method)?;
        let values = Self::values(input, marker)?;
        let table = Self::table(input, marker, data)?;
        let previous = table.insert(method.to_owned(), Value::Array(values));
        Self::require_first(input, marker, previous.as_ref())
    }

    pub fn record_scalars(
        input: &Input<'_>,
        marker: &Marker,
        method: &str,
        data: &mut Map<String, Value>,
    ) -> Result<(), Failure> {
        let values = Self::values(input, marker)?;
        Self::require_scalars(input, marker, &values)?;
        Self::record(input, marker, method, data)
    }

    pub fn inputs(data: &Map<String, Value>) -> Map<String, Value> {
        data.get(Self::DATA_KEY)
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default()
    }

    pub fn case_table(cases: &[String], inputs: &Map<String, Value>) -> Value {
        let rows = cases
            .iter()
            .flat_map(|method| Self::method_rows(method, inputs.get(method)))
            .collect();
        Value::Array(rows)
    }

    pub fn rust_calls(cases: &[String], inputs: &Map<String, Value>) -> Vec<(String, String)> {
        Self::calls(cases, inputs, &CallStyle::RUST)
    }

    pub fn jvm_calls(cases: &[String], inputs: &Map<String, Value>) -> Vec<(String, String)> {
        Self::calls(cases, inputs, &CallStyle::JVM)
    }

    pub fn csharp_calls(cases: &[String], inputs: &Map<String, Value>) -> Vec<(String, String)> {
        Self::calls(cases, inputs, &CallStyle::JSON)
    }

    pub fn calls(
        cases: &[String],
        inputs: &Map<String, Value>,
        style: &CallStyle,
    ) -> Vec<(String, String)> {
        cases
            .iter()
            .flat_map(|method| Self::method_calls(method, inputs.get(method), style))
            .collect()
    }

    pub fn require_case(input: &Input<'_>, marker: &Marker, method: &str) -> Result<(), Failure> {
        if Self::CASE_PREFIXES
            .iter()
            .any(|prefix| method.starts_with(prefix))
        {
            return Ok(());
        }
        Err(input.failure(marker.span.start, Self::NOT_A_CASE))
    }

    pub fn values(input: &Input<'_>, marker: &Marker) -> Result<Vec<Value>, Failure> {
        let listed = Self::list(input, marker)?;
        Self::bounded(input, marker, listed)
    }

    pub fn list(input: &Input<'_>, marker: &Marker) -> Result<Vec<Value>, Failure> {
        let parsed = crate::behavior_literal::parse(&marker.arguments).ok();
        let Some(Value::Array(values)) = parsed else {
            return Err(input.failure(marker.span.start, Self::NOT_A_LIST));
        };
        Ok(values)
    }

    pub fn bounded(
        input: &Input<'_>,
        marker: &Marker,
        values: Vec<Value>,
    ) -> Result<Vec<Value>, Failure> {
        if (1..=Self::MAX_INPUTS).contains(&values.len()) {
            return Ok(values);
        }
        Err(input.failure(marker.span.start, Self::NOT_A_LIST))
    }

    pub fn require_scalars(
        input: &Input<'_>,
        marker: &Marker,
        values: &[Value],
    ) -> Result<(), Failure> {
        if values.iter().all(Self::is_scalar) {
            return Ok(());
        }
        Err(input.failure(marker.span.start, Self::NOT_SCALAR))
    }

    pub fn is_scalar(value: &Value) -> bool {
        matches!(value, Value::Number(_) | Value::String(_) | Value::Bool(_))
    }

    pub fn table<'d>(
        input: &Input<'_>,
        marker: &Marker,
        data: &'d mut Map<String, Value>,
    ) -> Result<&'d mut Map<String, Value>, Failure> {
        data.entry(Self::DATA_KEY)
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .ok_or_else(|| input.failure(marker.span.start, Self::NOT_A_LIST))
    }

    pub fn require_first(
        input: &Input<'_>,
        marker: &Marker,
        previous: Option<&Value>,
    ) -> Result<(), Failure> {
        if previous.is_none() {
            return Ok(());
        }
        Err(input.failure(marker.span.start, Self::REPEATED))
    }

    pub fn method_rows(method: &str, values: Option<&Value>) -> Vec<Value> {
        let Some(values) = values.and_then(Value::as_array) else {
            return vec![serde_json::json!([method, method, []])];
        };
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                serde_json::json!([Self::case_name(method, index), method, [value]])
            })
            .collect()
    }

    pub fn method_calls(
        method: &str,
        values: Option<&Value>,
        style: &CallStyle,
    ) -> Vec<(String, String)> {
        let Some(values) = values.and_then(Value::as_array) else {
            return vec![(method.to_owned(), style.call(method, ""))];
        };
        Self::input_calls(method, values, style)
    }

    pub fn input_calls(method: &str, values: &[Value], style: &CallStyle) -> Vec<(String, String)> {
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let call = style.call(method, &(style.literal)(value));
                (Self::case_name(method, index), call)
            })
            .collect()
    }

    pub fn case_name(method: &str, index: usize) -> String {
        format!("{method}[{}]", index + 1)
    }
}
