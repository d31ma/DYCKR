#[derive(Clone, Default)]
pub struct JavaRuntimeSection {
    pub entries: Vec<(String, String)>,
}

impl JavaRuntimeSection {
    pub fn add(&mut self, label: String, text: String) {
        self.entries
            .extend((!text.is_empty()).then_some((label, text)));
    }

    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(label, _)| label.as_str())
    }

    pub fn joined(self, other: &Self) -> Option<Self> {
        other.entries.iter().try_fold(self, crate::java_runtime_section::JavaRuntimeSection::with)
    }

    pub fn text(&self) -> String {
        self.entries.iter().map(|(_, text)| text.as_str()).collect()
    }

    pub fn with(mut self, (label, text): &(String, String)) -> Option<Self> {
        let same = self
            .entries
            .iter()
            .find(|(known, _)| known == label)
            .map(|(_, known)| known == text);
        if let Some(same) = same {
            return same.then_some(self);
        }
        self.entries.push((label.clone(), text.clone()));
        Some(self)
    }
}


