#[derive(Debug, Default, Clone)]
pub struct ShadowLines {
    lines: Vec<String>,
    sources: Vec<usize>,
}

impl ShadowLines {
    pub fn push(&mut self, text: impl Into<String>, source_line: usize) {
        self.lines.push(text.into());
        self.sources.push(source_line);
    }

    pub fn text(&self) -> String {
        let mut text = self.lines.join("\n");
        text.push('\n');
        text
    }

    pub fn source_line(&self, shadow_line: usize) -> Option<usize> {
        self.sources.get(shadow_line).copied()
    }
}
