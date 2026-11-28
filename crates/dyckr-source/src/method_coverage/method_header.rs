use super::{HookStatement, PublicMethod, RoleClass};

pub struct MethodHeader<'a> {
    pub file: &'a str,
    pub masked: &'a str,
    pub source: &'a str,
    pub start: usize,
    pub name: String,
    pub key: String,
    pub hook: String,
}

impl<'a> MethodHeader<'a> {
    pub fn new(role: &'a RoleClass, source: &'a str, method: &PublicMethod) -> Self {
        let key = role.key(&method.name);
        Self {
            file: &role.file,
            masked: &role.masked,
            source,
            start: Self::line_start(&role.masked, method.line),
            name: method.name.clone(),
            hook: HookStatement::statement(&role.language, &key),
            key,
        }
    }

    pub fn line(&self) -> &'a str {
        let rest = &self.masked[self.start..];
        rest.lines().next().unwrap_or_default()
    }

    pub fn line_end(&self) -> usize {
        self.start + self.line().len()
    }

    pub fn name_end(&self) -> Option<usize> {
        let found = self.line().find(&self.name)?;
        Some(self.start + found + self.name.len())
    }

    pub fn parameters_end(&self) -> Option<usize> {
        let name_end = self.name_end()?;
        let open = name_end + self.masked[name_end..].find('(')?;
        Self::closing(self.masked, open)
    }

    pub fn closing(masked: &str, open: usize) -> Option<usize> {
        let depths = Self::depths(&masked[open..]);
        let close = depths.iter().position(|depth| *depth == 0)?;
        Some(open + close + 1)
    }

    pub fn depths(text: &str) -> Vec<i64> {
        text.bytes()
            .scan(0_i64, |depth, byte| {
                *depth += Self::depth_change(byte);
                Some(*depth)
            })
            .collect()
    }

    pub fn find_at_depth_zero(text: &str, wanted: u8) -> Option<usize> {
        let depths = Self::depths(text);
        text.bytes()
            .zip(depths)
            .position(|pair| pair == (wanted, 0))
    }

    pub fn first_after(&self, from: usize, wanted: &[char]) -> Option<usize> {
        let found = self.masked[from..].find(wanted)?;
        Some(from + found)
    }

    fn depth_change(byte: u8) -> i64 {
        match byte {
            b'(' | b'[' | b'{' => 1,
            b')' | b']' | b'}' => -1,
            _ => 0,
        }
    }

    fn line_start(masked: &str, line: usize) -> usize {
        masked.split_inclusive('\n').take(line).map(str::len).sum()
    }
}
