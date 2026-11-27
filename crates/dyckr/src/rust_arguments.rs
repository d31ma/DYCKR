use std::fmt::Write as _;


pub fn rust_arguments(parameters: &[String]) -> (String, String) {
    let mut bindings = String::new();
    let mut passed = Vec::new();
    for (index, kind) in parameters.iter().enumerate() {
        let conversion = crate::rust_model_type::rust_write(kind).unwrap_or_else(|| String::from("value.as_text()"));
        let _ = write!(
            bindings,
            "let a{index} = {{ let value = arguments.next().unwrap_or(TacValue::Null); {conversion} }}; "
        );
        if kind == "&str" {
            passed.push(format!("&a{index}"));
        } else {
            passed.push(format!("a{index}"));
        }
    }
    (bindings, passed.join(", "))
}
