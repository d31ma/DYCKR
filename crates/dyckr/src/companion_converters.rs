use std::fmt::Write as _;


pub fn swift_constructor(fields: &[(String, String)]) -> String {
    let mut constructor = String::from("\ninit?(tacProps: [String: Any]) {\n");
    for (name, kind) in fields {
        let _ = std::writeln!(
            constructor,
            "guard let {name} = tachyonSwiftJSONValue(tacProps[\"{name}\"]) as? {kind} else {{ return nil }}\nself.{name} = {name}"
        );
    }
    constructor.push_str("}\n");
    constructor
}

pub fn kotlin_constructor(fields: &[(String, String)]) -> String {
    let mut constructor = String::from(
        "\ninternal constructor(tacProps: Map<String, Any?>?, tacPage: TacPageScope) {\n",
    );
    for (name, kind) in fields {
        let value = format!("tacRequiredValue(tacProps, \"{name}\")");
        let converted = match kind.as_str() {
            "DocumentTitle" => String::from("tacPage.documentTitle"),
            "Int" | "Long" | "Double" | "Float" | "Boolean" | "String" => {
                format!("{value}.let {{ {} }}", kotlin_value(kind, "it"))
            }
            _ => kotlin_value(kind, &value),
        };
        let _ = std::writeln!(constructor, "this.{name} = {converted}");
    }
    constructor.push_str("}\n");
    constructor
}

pub fn kotlin_value(kind: &str, value: &str) -> String {
    match kind {
        "Int" => format!("tacInt({value})"),
        "Long" => format!("tacLong({value})"),
        "Double" => format!("tacDouble({value})"),
        "Boolean" => format!("tacBoolean({value})"),
        "String" => format!("tacString({value})"),
        _ => format!("tacRead<{kind}>({value})"),
    }
}

pub fn csharp_converter(kind: &str) -> &'static str {
    match kind {
        "int" => "Convert.ToInt32(value)",
        "long" => "Convert.ToInt64(value)",
        "double" | "float" => "Convert.ToDouble(value)",
        "decimal" => "Convert.ToDecimal(value)",
        "bool" => "Convert.ToBoolean(value)",
        "object" => "value",
        _ => "Convert.ToString(value)",
    }
}
