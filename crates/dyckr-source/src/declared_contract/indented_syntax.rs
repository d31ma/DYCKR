#[derive(Clone, Copy)]
pub struct IndentedSyntax {
    pub ruby: bool,
    pub marker_prefix: &'static str,
    pub methods: &'static [&'static str],
    pub inline_bodies: bool,
    pub closing: &'static str,
}

impl IndentedSyntax {
    const SYNTAXES: [Self; 2] = [
        Self {
            ruby: false,
            marker_prefix: "@",
            methods: &["def ", "async def "],
            inline_bodies: true,
            closing: "\n",
        },
        Self {
            ruby: true,
            marker_prefix: "#[",
            methods: &["def "],
            inline_bodies: false,
            closing: "end",
        },
    ];

    pub fn of(ruby: bool) -> Self {
        Self::SYNTAXES[usize::from(ruby)]
    }
}
