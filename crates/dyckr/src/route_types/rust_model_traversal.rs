use std::collections::BTreeSet;

#[derive(Default)]
pub struct RustModelTraversal {
    visited_records: BTreeSet<String>,
    visited_aliases: BTreeSet<String>,
    active_aliases: BTreeSet<String>,
}

impl RustModelTraversal {
    pub fn count(&self) -> usize {
        self.visited_records.len() + self.visited_aliases.len()
    }

    pub fn knows_record(&self, name: &str) -> bool {
        self.visited_records.contains(name)
    }

    pub fn knows_alias(&self, name: &str) -> bool {
        self.visited_aliases.contains(name)
    }

    pub fn enter_record(&mut self, name: &str) -> bool {
        self.visited_records.insert(name.to_owned())
    }

    pub fn enter_alias(&mut self, name: &str) -> bool {
        self.visited_aliases.insert(name.to_owned());
        self.active_aliases.insert(name.to_owned())
    }

    pub fn leave_alias(&mut self, name: &str) {
        self.active_aliases.remove(name);
    }

    pub fn for_fields(&self) -> Self {
        Self {
            visited_records: self.visited_records.clone(),
            visited_aliases: self.visited_aliases.clone(),
            active_aliases: BTreeSet::new(),
        }
    }

    pub fn finish_fields(&mut self, fields: Self) {
        self.visited_records.extend(fields.visited_records);
        self.visited_aliases.extend(fields.visited_aliases);
    }
}
