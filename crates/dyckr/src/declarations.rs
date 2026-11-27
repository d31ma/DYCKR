pub struct Declarations;

impl Declarations {
    pub fn variants(source: &str, name: &str) -> Option<Vec<String>> {
        Self::enum_variants(source, name)
    }

    pub fn enum_variants(source: &str, name: &str) -> Option<Vec<String>> {
        let header = format!("enum {name}");
        let at = source.find(&header)?;
        let open = at + source[at..].find('{')?;
        let close = open + source[open..].find('}')?;
        source[open + 1..close]
            .split([',', ';'])
            .map(|part| part.split('=').next().unwrap_or("").trim())
            .filter(|variant| !variant.is_empty())
            .map(Self::variant)
            .collect()
    }

    pub fn variant(text: &str) -> Option<String> {
        text.bytes()
            .all(Self::is_word_byte)
            .then(|| text.to_owned())
    }

    pub fn is_word_byte(byte: u8) -> bool {
        [byte.is_ascii_alphanumeric(), byte == b'_'].contains(&true)
    }
}
