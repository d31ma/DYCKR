pub fn camel_case(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut capitalise = false;
    for character in value.chars() {
        if character == '_' {
            capitalise = true;
        } else if capitalise {
            output.extend(character.to_uppercase());
            capitalise = false;
        } else {
            output.push(character);
        }
    }
    output
}
