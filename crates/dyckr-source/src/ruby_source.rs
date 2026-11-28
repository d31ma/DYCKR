use crate::Failure;
use dyckr::ascii_name::AsciiName;
use dyckr::go_marker_name::GoMarkerName;
use dyckr::ruby_lexer::RubyLexer;
pub use needed_field::NeededField;
pub use ruby_needs::RubyNeeds;
use std::path::Path;
pub use untyped_field::UntypedField;

mod needed_field;
mod ruby_needs;
mod untyped_field;

pub struct RubyMarkers;

impl RubyMarkers {
    const MAX_NAME: usize = 128;
    const HELP: &'static str =
        "Write each Ruby annotation as `#[Name]` on its own line before the declaration.";

    pub fn validate(path: &Path, source: &str, code: &str) -> Result<(), Failure> {
        if let Some((at, message)) = RubyLexer::code_mask(source).marker_problem {
            return Err(Self::failure(path, source, at, message));
        }
        let mut offset = 0;
        for line in code.split_inclusive('\n') {
            let trimmed = line.trim_start();
            let at = offset + line.len() - trimmed.len();
            offset += line.len();
            if let Some(rest) = trimmed.strip_prefix("#[") {
                if let Some(message) = Self::refusal(GoMarkerName::read(rest)) {
                    return Err(Self::failure(path, source, at, message));
                }
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix('@')
                && Self::is_bare_marker(rest)
            {
                let name = GoMarkerName::read(rest);
                let message = format!(
                    "Ruby annotations are comments: write `#[{name}]` on its own line, not `@{name}`"
                );
                return Err(Self::failure(path, source, at, message));
            }
        }
        Ok(())
    }

    fn is_bare_marker(rest: &str) -> bool {
        let name = GoMarkerName::read(rest);
        let after = rest[name.len()..].trim_end();
        let whole_line = after.is_empty() || (after.starts_with('(') && after.ends_with(')'));
        let capitalized = name.as_bytes().first().is_some_and(u8::is_ascii_uppercase);
        whole_line && capitalized && GoMarkerName::known(name)
    }

    fn refusal(name: &str) -> Option<String> {
        if GoMarkerName::known(name) {
            return None;
        }
        let written: String = name.chars().take(Self::MAX_NAME).collect();
        let (namespace, plain) = match written.rsplit_once("::") {
            Some((namespace, plain)) => (format!("{namespace}::"), plain.to_owned()),
            None => (String::new(), written.clone()),
        };
        let lowered = namespace.to_ascii_lowercase();
        if namespace != lowered {
            return Some(format!(
                "unknown annotation `#[{written}]`; namespaces are lowercase: write `#[{lowered}{plain}]`"
            ));
        }
        let help = GoMarkerName::suggestion(&plain).map_or_else(String::new, |candidate| {
            format!("; did you mean `#[{namespace}{candidate}]`?")
        });
        Some(format!("unknown annotation `#[{written}]`{help}"))
    }

    fn failure(path: &Path, source: &str, at: usize, message: impl Into<String>) -> Failure {
        Failure::one(dyckr::failure::diagnostic(
            1308,
            message,
            Some(String::from(Self::HELP)),
            dyckr::failure::source_span(&path.to_string_lossy(), at, (at + 1).min(source.len())),
        ))
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RubyMembers {
    pub fields: Vec<String>,
    pub writable: Vec<String>,
    pub methods: Vec<String>,
}

impl RubyMembers {
    const HIDING_SECTIONS: [&'static str; 2] = ["private", "protected"];

    #[must_use]
    pub fn without_needed(mut self, source: &str) -> Self {
        let needed = RubyNeeds::fields(source);
        self.fields.retain(|field| !needed.contains(field));
        self.writable.retain(|field| !needed.contains(field));
        self
    }

    pub fn read(code: &str) -> Self {
        let mut members = Self::default();
        let mut member_indent = None;
        let mut hidden = false;
        for line in code.lines() {
            let text = line.trim();
            if text.is_empty() {
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            if indent == 0 {
                hidden = false;
                member_indent = None;
                continue;
            }
            if indent != *member_indent.get_or_insert(indent) {
                continue;
            }
            if Self::HIDING_SECTIONS.contains(&text) {
                hidden = true;
            } else if text == "public" {
                hidden = false;
            } else if !hidden {
                members.read_member(text);
            }
        }
        members
    }

    fn read_member(&mut self, text: &str) {
        if let Some(rest) = text.strip_prefix("attr_reader ") {
            Self::add_all(&mut self.fields, rest);
            return;
        }
        if let Some(rest) = text.strip_prefix("attr_accessor ") {
            Self::add_all(&mut self.fields, rest);
            Self::add_all(&mut self.writable, rest);
            return;
        }
        let Some(rest) = text.strip_prefix("def ") else {
            return;
        };
        let name = AsciiName::leading(rest);
        let plain_end = rest[name.len()..]
            .chars()
            .next()
            .is_none_or(|next| matches!(next, '(' | ' ' | ';'));
        let hidden = name.is_empty() || name == "initialize" || name.starts_with('_');
        if plain_end && !hidden && !self.methods.iter().any(|known| known == name) {
            self.methods.push(name.to_owned());
        }
    }

    fn add_all(names: &mut Vec<String>, symbols: &str) {
        for part in symbols.split(',') {
            let Some(name) = part.trim().strip_prefix(':') else {
                continue;
            };
            let valid = !name.is_empty()
                && name
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
            if valid && !names.iter().any(|known| known == name) {
                names.push(name.to_owned());
            }
        }
    }
}

#[cfg(test)]
mod member_tests {
    use super::{RubyLexer, RubyMembers};

    #[test]
    fn public_readers_accessors_and_methods_form_the_member_table() {
        let source = "class CounterService\n  attr_reader :count, :label\n  attr_accessor :step\n\n  def initialize\n    @count = 0\n  end\n\n  def increment(by = 1)\n    @count += by\n  end\n\n  def self.build\n  end\n\n  def empty?\n  end\n\n  private\n\n  def hidden\n  end\nend\n";
        let mask = RubyLexer::code_mask(source);
        let code: String = source
            .char_indices()
            .map(|(at, value)| if mask.code[at] { value } else { ' ' })
            .collect();
        let members = RubyMembers::read(&code);
        assert_eq!(members.fields, ["count", "label", "step"]);
        assert_eq!(members.writable, ["step"]);
        assert_eq!(members.methods, ["increment"]);
    }
}

#[cfg(test)]
mod yard_tests {

    use dyckr::ascii_name::AsciiName;

    fn texts() -> Vec<String> {
        let alphabet = (0..=0x7f_u8)
            .map(char::from)
            .chain(['é', 'Ü', 'ß', '٣'])
            .collect::<Vec<_>>();
        let mut texts = vec![String::new()];
        for first in &alphabet {
            texts.push(first.to_string());
            texts.extend(alphabet.iter().map(|second| format!("{first}{second}")));
        }
        texts
    }

    fn leading_word(text: &str) -> &str {
        let length = text
            .bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
            .count();
        &text[..length]
    }

    #[test]
    fn yard_words_skip_leading_space_then_read_ascii_word_characters() {
        for text in texts() {
            for padded in [text.clone(), format!("  {text}"), format!("\t{text}")] {
                let expected = leading_word(padded.trim_start());
                assert_eq!(
                    dyckr::yard::Yard::first_word(&padded),
                    expected,
                    "{padded:?}"
                );
            }
            assert_eq!(AsciiName::leading(&text), leading_word(&text), "{text:?}");
        }
    }

    #[test]
    fn parameter_and_return_types_come_from_yard_tags() {
        let source = "class A\n  # Finds an order.\n  # @param id [Integer] the order number\n  # @param [String, nil] note\n  #[GetRoute(\"/{id}\")]\n  # @return [Array<Order>] the orders\n  def get_order(id, note)\n";
        let declaration = source.find("def").unwrap_or_default();
        let comments = dyckr::yard::Yard::comments_before(source, declaration);
        assert_eq!(
            dyckr::yard::Yard::parameter_type(&comments, "id"),
            "Integer"
        );
        assert_eq!(
            dyckr::yard::Yard::parameter_type(&comments, "note"),
            "String, nil"
        );
        assert_eq!(dyckr::yard::Yard::parameter_type(&comments, "other"), "");
        assert_eq!(dyckr::yard::Yard::return_type(&comments), "Array<Order>");
        assert!(!comments.contains("GetRoute"), "{comments}");
    }

    #[test]
    fn attribute_directives_pair_names_with_types() {
        let comments = " @!attribute [r] id\n   @return [String]\n @!attribute [r] lines\n   @return [Array<Line>]";
        assert_eq!(
            dyckr::yard::Yard::attribute_types(comments),
            [
                (String::from("id"), String::from("String")),
                (String::from("lines"), String::from("Array<Line>")),
            ]
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod marker_tests {
    use super::RubyMarkers;
    use std::path::Path;

    fn problem(source: &str) -> Option<String> {
        let code = dyckr::source_mask::masked_source(Path::new("server/services/A.rb"), source)
            .expect("masked");
        RubyMarkers::validate(Path::new("server/services/A.rb"), source, &code)
            .err()
            .map(|failure| failure.to_string())
    }

    #[test]
    fn known_markers_and_ordinary_comments_pass() {
        let source = "#[Service]\nclass AService\n  # [x] a plain note\n  #[deprecated]\n  def a; end\nend\n";
        assert_eq!(problem(source), None);
    }

    #[test]
    fn near_misses_and_bare_markers_are_refused() {
        let typo = problem("#[Servce]\nclass AService\nend\n").expect("typo");
        assert!(typo.contains("did you mean `#[Service]`"), "{typo}");
        let bare = problem("@Service\nclass AService\nend\n").expect("bare");
        assert!(bare.contains("write `#[Service]`"), "{bare}");
        let spaced = problem("# [Service]\nclass AService\nend\n").expect("spaced");
        assert!(spaced.contains("RuboCop"), "{spaced}");
        let namespace = problem("#[Yon::Service]\nclass AService\nend\n").expect("namespace");
        assert!(namespace.contains("#[yon::Service]"), "{namespace}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use dyckr::ruby_lexer::RubyLexer;

    fn visible_code(source: &str) -> String {
        let mask = RubyLexer::code_mask(source);
        assert!(mask.error.is_none(), "{source}: {:?}", mask.error);
        source
            .char_indices()
            .map(|(at, value)| if mask.code[at] { value } else { ' ' })
            .collect()
    }

    #[test]
    fn comments_are_hidden_but_marker_lines_stay_visible() {
        let source = "# don't \"stop\"\n#[Service]\nclass Store # it's here\n  #[GetRoute(\"/{id}\")]\n  def get_item; end\nend\n";
        let masked = visible_code(source);
        assert!(masked.contains("#[Service]"), "{masked}");
        assert!(masked.contains("#[GetRoute("), "{masked}");
        assert!(
            !masked.contains("don't") && !masked.contains("it's"),
            "{masked}"
        );
        assert!(!masked.contains("/{id}"), "{masked}");
        assert!(masked.contains("class Store") && masked.contains("def get_item"));
    }

    #[test]
    fn ordinary_bracket_comments_and_inline_markers_stay_comments() {
        let masked = visible_code("#[deprecated]\nx = 1 #[Service]\n");
        assert!(!masked.contains("deprecated"), "{masked}");
        assert!(!masked.contains("Service"), "{masked}");
    }

    #[test]
    fn heredocs_and_block_comments_hide_marker_lookalikes() {
        let source = "text = <<~TEXT\n  #[Service]\n  it's\nTEXT\n=begin\n#[Service]\n=end\nsql = <<-'SQL' + <<EOS\nx\n  SQL\n#[Store]\nEOS\n#[Delegate]\nclass A; end\n";
        let masked = visible_code(source);
        assert!(
            !masked.contains("Service") && !masked.contains("Store"),
            "{masked}"
        );
        assert!(
            masked.contains("#[Delegate]") && masked.contains("class A"),
            "{masked}"
        );
        assert!(masked.contains("text = <<~TEXT"), "{masked}");
    }

    #[test]
    fn interpolation_percent_literals_and_regular_expressions_are_hidden() {
        let source = "a = \"x #{h[\"}\"] + '{'} y\"; GET()\nb = %w[it's #{x}] ; c = %q(a (b) ') ; POST()\nd = text.split /'/ ; e = 4 / 2 ; f = ?' ; PUT()\ng = $' ; h = x % 3 ; DELETE()\n";
        let masked = visible_code(source);
        for call in ["GET()", "POST()", "PUT()", "DELETE()"] {
            assert!(masked.contains(call), "{call}: {masked}");
        }
        assert!(
            masked.contains("4 / 2") && masked.contains("x % 3"),
            "{masked}"
        );
        assert!(
            !masked.contains("it's") && !masked.contains('}'),
            "{masked}"
        );
    }

    #[test]
    fn a_spaced_marker_is_reported_as_a_rubocop_change() {
        let mask = RubyLexer::code_mask("# [Service]\nclass Store\nend\n# [Order] is fine too\n");
        assert_eq!(mask.marker_problem.map(|(at, _)| at), Some(0));
        let plain = RubyLexer::code_mask("# [Order] the first order\n# @return [String]\n");
        assert!(plain.marker_problem.is_none());
    }

    #[test]
    fn unterminated_literals_are_errors() {
        for source in [
            "x = \"open\n",
            "=begin\nnever closed\n",
            "x = <<~EOS\nbody\n",
            "x = %w[a b\n",
        ] {
            assert!(RubyLexer::code_mask(source).error.is_some(), "{source}");
        }
    }

    #[test]
    fn data_after_end_marker_is_hidden() {
        let masked = visible_code("class A\nend\n__END__\n#[Service] it's data\n");
        assert!(!masked.contains("Service"), "{masked}");
    }
}
