use crate::lexical_mask::LexicalMask;

pub struct RubyLexer<'a> {
    bytes: &'a [u8],
    source: &'a str,
    code: Vec<bool>,
    heredocs: Vec<Heredoc>,
    marker_problem: Option<(usize, &'static str)>,
}

pub struct Heredoc {
    label: String,
    indented: bool,
}

pub type LexicalError = (usize, &'static str);

impl<'a> RubyLexer<'a> {
    pub const SPACED_MARKER: &'static str = "Write Ruby annotations as `#[Name]`, with no space after `#`. RuboCop's Layout/LeadingCommentSpace rule adds that space; turn the rule off in .rubocop.yml";
    const EXPRESSION_KEYWORDS: [&'static str; 18] = [
        "if", "unless", "while", "until", "when", "case", "and", "or", "not", "return", "then",
        "else", "elsif", "in", "do", "puts", "print", "yield",
    ];
    const PERCENT_KINDS: &'static [u8] = b"qQwWiIrsx";
    const PERCENT_DELIMITERS: &'static [u8] = b"([{<|!/^";
    const CHARACTER_LITERALS: &'static [u8] = b"'\"`#/%<?\\";

    pub fn code_mask(source: &'a str) -> LexicalMask {
        let mut lexer = Self {
            bytes: source.as_bytes(),
            source,
            code: vec![true; source.len()],
            heredocs: Vec::new(),
            marker_problem: None,
        };
        let error = lexer.scan().err();
        let mut code = lexer.code;
        let error = error.map(|(start, message)| {
            code[start..].fill(false);
            message
        });
        LexicalMask {
            code,
            error,
            marker_problem: lexer.marker_problem,
        }
    }

    fn scan(&mut self) -> Result<(), LexicalError> {
        let mut at = 0;
        while at < self.bytes.len() {
            at = self.step(at)?;
        }
        if let Some(heredoc) = self.heredocs.first() {
            let start = self.source.rfind(&heredoc.label).unwrap_or(0);
            return Err((start, "Unterminated Ruby heredoc"));
        }
        Ok(())
    }

    fn step(&mut self, at: usize) -> Result<usize, LexicalError> {
        let byte = self.bytes[at];
        if byte == b'\n' {
            return self.after_newline(at);
        }
        if self.at_line_start(at) {
            if let Some(end) = self.block_comment(at)? {
                return Ok(self.hide(at, end));
            }
            if self.line_text(at) == "__END__" {
                return Ok(self.hide(at, self.bytes.len()));
            }
        }
        match byte {
            b'#' => Ok(self.comment(at)),
            b'"' | b'`' => {
                let end = self
                    .interpolated_end(at + 1, byte)
                    .ok_or((at, "Unterminated Ruby string"))?;
                Ok(self.hide(at, end))
            }
            b'\'' => {
                let end = self
                    .escaped_end(at + 1, b'\'')
                    .ok_or((at, "Unterminated Ruby string"))?;
                Ok(self.hide(at, end))
            }
            b'%' => self.percent_literal(at),
            b'<' => Ok(self.heredoc_start(at)),
            b'?' => Ok(self.character_literal(at)),
            b'/' => Ok(self.regular_expression(at)),
            b'$' => Ok(self.global_variable(at)),
            _ => Ok(at + 1),
        }
    }

    fn hide(&mut self, start: usize, end: usize) -> usize {
        let end = end.min(self.bytes.len());
        self.code[start..end].fill(false);
        end
    }

    fn at_line_start(&self, at: usize) -> bool {
        at == 0 || self.bytes[at - 1] == b'\n'
    }

    fn line_end(&self, at: usize) -> usize {
        self.source[at..]
            .find('\n')
            .map_or(self.bytes.len(), |offset| at + offset)
    }

    fn line_start(&self, at: usize) -> usize {
        self.source[..at].rfind('\n').map_or(0, |offset| offset + 1)
    }

    fn line_text(&self, at: usize) -> &str {
        self.source[at..self.line_end(at)].trim_end_matches('\r')
    }

    fn starts_word(line: &str, word: &str) -> bool {
        let Some(rest) = line.strip_prefix(word) else {
            return false;
        };
        rest.is_empty() || rest.starts_with([' ', '\t'])
    }

    fn block_comment(&self, at: usize) -> Result<Option<usize>, LexicalError> {
        if !Self::starts_word(self.line_text(at), "=begin") {
            return Ok(None);
        }
        let mut line = self.line_end(at) + 1;
        while line < self.bytes.len() {
            let end = self.line_end(line);
            if Self::starts_word(self.line_text(line), "=end") {
                return Ok(Some(end));
            }
            line = end + 1;
        }
        Err((at, "Unterminated Ruby =begin comment"))
    }

    fn comment(&mut self, at: usize) -> usize {
        let end = self.line_end(at);
        let first_on_line = self.source[self.line_start(at)..at].trim().is_empty();
        if first_on_line && self.bytes.get(at + 1) == Some(&b'[') {
            let text = &self.source[at + 2..end];
            if crate::go_marker_name::GoMarkerName::is_marker(text) {
                return at + 1;
            }
        }
        if first_on_line {
            self.check_spaced_marker(at, end);
        }
        self.hide(at, end)
    }

    fn check_spaced_marker(&mut self, at: usize, end: usize) {
        let after_hash = &self.source[at + 1..end];
        let Some(rest) = after_hash.trim_start_matches([' ', '\t']).strip_prefix('[') else {
            return;
        };
        if after_hash.len() == rest.len() + 1 {
            return;
        }
        let closes_line = rest.trim_end().ends_with(']');
        if closes_line && crate::go_marker_name::GoMarkerName::is_marker(rest) {
            self.marker_problem = self.marker_problem.or(Some((at, Self::SPACED_MARKER)));
        }
    }

    fn after_newline(&mut self, at: usize) -> Result<usize, LexicalError> {
        if self.heredocs.is_empty() {
            return Ok(at + 1);
        }
        let mut line = at + 1;
        for heredoc in std::mem::take(&mut self.heredocs) {
            let end = self
                .heredoc_end(line, &heredoc)
                .ok_or((at, "Unterminated Ruby heredoc"))?;
            self.hide(line, end);
            line = end + 1;
        }
        Ok(line.saturating_sub(1).max(at + 1))
    }

    fn heredoc_end(&self, mut line: usize, heredoc: &Heredoc) -> Option<usize> {
        while line < self.bytes.len() {
            let end = self.line_end(line);
            let text = self.line_text(line);
            let text = if heredoc.indented {
                text.trim_start()
            } else {
                text
            };
            if text == heredoc.label {
                return Some(end);
            }
            line = end + 1;
        }
        None
    }

    fn heredoc_start(&mut self, at: usize) -> usize {
        let repeated = at > 0 && self.bytes[at - 1] == b'<';
        if self.bytes.get(at + 1) != Some(&b'<') || repeated {
            return at + 1;
        }
        let mut next = at + 2;
        let indented = matches!(self.bytes.get(next), Some(b'~' | b'-'));
        if indented {
            next += 1;
        }
        let Some(&first) = self.bytes.get(next) else {
            return at + 1;
        };
        if matches!(first, b'\'' | b'"' | b'`') {
            let line_end = self.line_end(next);
            let Some(close) = self.source[next + 1..line_end].find(first as char) else {
                return at + 1;
            };
            let label = self.source[next + 1..next + 1 + close].to_owned();
            self.heredocs.push(Heredoc { label, indented });
            return next + close + 2;
        }
        let starts_label = if indented {
            first.is_ascii_alphabetic() || first == b'_'
        } else {
            first.is_ascii_uppercase() || first == b'_'
        };
        if !starts_label {
            return at + 1;
        }
        let mut end = next;
        while end < self.bytes.len() && Self::is_word_byte(self.bytes[end]) {
            end += 1;
        }
        let label = self.source[next..end].to_owned();
        self.heredocs.push(Heredoc { label, indented });
        end
    }

    fn percent_literal(&mut self, at: usize) -> Result<usize, LexicalError> {
        let Some(&first) = self.bytes.get(at + 1) else {
            return Ok(at + 1);
        };
        let delimiter_at = if Self::PERCENT_KINDS.contains(&first) {
            at + 2
        } else if Self::PERCENT_DELIMITERS.contains(&first) && self.expression_start(at) {
            at + 1
        } else {
            return Ok(at + 1);
        };
        let Some(&delimiter) = self.bytes.get(delimiter_at) else {
            return Ok(at + 1);
        };
        let is_delimiter = delimiter.is_ascii_punctuation() && delimiter != b'=';
        if !is_delimiter {
            return Ok(at + 1);
        }
        let end = self
            .delimited_end(delimiter_at + 1, delimiter)
            .ok_or((at, "Unterminated Ruby percent literal"))?;
        Ok(self.hide(at, end))
    }

    fn delimited_end(&self, mut at: usize, opening: u8) -> Option<usize> {
        let closing = match opening {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            b'<' => b'>',
            other => other,
        };
        let nests = closing != opening;
        let mut depth = 0_usize;
        while at < self.bytes.len() {
            let byte = self.bytes[at];
            if byte == b'\\' {
                at += 2;
                continue;
            }
            if nests && byte == opening {
                depth += 1;
            } else if byte == closing && depth == 0 {
                return Some(at + 1);
            } else if byte == closing {
                depth -= 1;
            }
            at += 1;
        }
        None
    }

    fn character_literal(&mut self, at: usize) -> usize {
        let Some(&next) = self.bytes.get(at + 1) else {
            return at + 1;
        };
        if !Self::CHARACTER_LITERALS.contains(&next) || !self.expression_start(at) {
            return at + 1;
        }
        let length = if next == b'\\' { 3 } else { 2 };
        let after = self.bytes.get(at + length).copied();
        if after.is_some_and(Self::is_word_byte) {
            return at + 1;
        }
        self.hide(at, at + length)
    }

    fn regular_expression(&mut self, at: usize) -> usize {
        if !self.starts_regular_expression(at) {
            return at + 1;
        }
        let mut position = at + 1;
        let mut in_class = false;
        while position < self.bytes.len() {
            match self.bytes[position] {
                b'\\' => position += 2,
                b'\n' => return at + 1,
                b'[' => {
                    in_class = true;
                    position += 1;
                }
                b']' => {
                    in_class = false;
                    position += 1;
                }
                b'/' if !in_class => return self.hide(at, position + 1),
                _ => position += 1,
            }
        }
        at + 1
    }

    fn starts_regular_expression(&self, at: usize) -> bool {
        if self.expression_start(at) {
            return true;
        }
        let after = self.bytes.get(at + 1).copied();
        let spaced_before = at > 0 && matches!(self.bytes[at - 1], b' ' | b'\t');
        let word_before = self.previous_word(at).is_some();
        spaced_before
            && word_before
            && after.is_some_and(|byte| !matches!(byte, b' ' | b'\t' | b'='))
    }

    fn global_variable(&self, at: usize) -> usize {
        let next = self.bytes.get(at + 1).copied();
        if next.is_some_and(|byte| byte.is_ascii_punctuation()) {
            return at + 2;
        }
        at + 1
    }

    fn previous_word(&self, at: usize) -> Option<&str> {
        let before = self.source[..at].trim_end_matches([' ', '\t']);
        let start = before
            .bytes()
            .rposition(|byte| !Self::is_word_byte(byte))
            .map_or(0, |position| position + 1);
        let word = &before[start..];
        (!word.is_empty()).then_some(word)
    }

    fn expression_start(&self, at: usize) -> bool {
        let before = self.source[..at].trim_end_matches([' ', '\t']);
        let Some(&last) = before.as_bytes().last() else {
            return true;
        };
        if last == b'\n' {
            return true;
        }
        if !Self::is_word_byte(last) {
            return !matches!(last, b')' | b']' | b'}' | b'"' | b'\'' | b'`');
        }
        self.previous_word(at)
            .is_some_and(|word| Self::EXPRESSION_KEYWORDS.contains(&word))
    }

    fn is_word_byte(byte: u8) -> bool {
        byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80
    }

    fn escaped_end(&self, mut at: usize, quote: u8) -> Option<usize> {
        while at < self.bytes.len() {
            match self.bytes[at] {
                b'\\' => at += 2,
                byte if byte == quote => return Some(at + 1),
                _ => at += 1,
            }
        }
        None
    }

    fn interpolated_end(&self, mut at: usize, quote: u8) -> Option<usize> {
        while at < self.bytes.len() {
            match self.bytes[at] {
                b'\\' => at += 2,
                b'#' if self.bytes.get(at + 1) == Some(&b'{') => {
                    at = self.interpolation_end(at + 2)?;
                }
                byte if byte == quote => return Some(at + 1),
                _ => at += 1,
            }
        }
        None
    }

    fn interpolation_end(&self, mut at: usize) -> Option<usize> {
        let mut depth = 1_usize;
        while at < self.bytes.len() {
            match self.bytes[at] {
                b'{' => {
                    depth += 1;
                    at += 1;
                }
                b'}' => {
                    depth -= 1;
                    at += 1;
                    if depth == 0 {
                        return Some(at);
                    }
                }
                b'"' | b'`' => at = self.interpolated_end(at + 1, self.bytes[at])?,
                b'\'' => at = self.escaped_end(at + 1, b'\'')?,
                _ => at += 1,
            }
        }
        None
    }
}
