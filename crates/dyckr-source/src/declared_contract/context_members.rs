use super::context_template::ContextTemplate;
use super::member_indent::MemberIndent;

pub struct ContextMembers<'a> {
    pub language: &'a str,
    pub tac: bool,
    pub provided: bool,
    pub needs_context: bool,
    pub owner: &'a str,
    pub receiver: &'a str,
    pub setup: &'a str,
    pub partition: &'a str,
    pub indent: &'a str,
}

impl ContextMembers<'_> {
    const REALMS: [&'static str; 2] = ["yon", "tac"];
    const BARE: &'static str = "bare";
    const PROVIDED: &'static str = "provided";

    pub fn parts_for(&self, contract: &str) -> (String, String) {
        let template = ContextTemplate::for_contract(contract, &self.keys());
        self.template_parts(template)
    }

    fn template_parts(&self, template: ContextTemplate) -> (String, String) {
        let field = self.filled(template.field);
        let members = self.filled(template.members);
        let field = Some(field)
            .filter(|field| !field.is_empty())
            .map(|field| format!(" {field}"))
            .unwrap_or_default();
        let members = Some(members)
            .filter(|members| !members.is_empty())
            .map(|members| {
                MemberIndent {
                    indent: self.indent,
                }
                .block(&members)
            })
            .unwrap_or_default();
        (field, members)
    }

    fn keys(&self) -> Vec<String> {
        let realm = Self::REALMS[usize::from(self.tac)];
        let variant = [realm, Self::BARE][usize::from(!self.needs_context)];
        let variant = [variant, Self::PROVIDED][usize::from(self.provided)];
        vec![
            format!("{}-{variant}", self.language),
            self.language.to_owned(),
        ]
    }

    fn filled(&self, template: &str) -> String {
        template
            .replace("{setup}", self.setup)
            .replace("{partition}", self.partition)
            .replace("{owner}", self.owner)
            .replace("{receiver}", self.receiver)
    }
}
