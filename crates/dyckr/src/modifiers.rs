pub struct Modifiers;

impl Modifiers {
    pub const CLASS: [&str; 11] = [
        "public",
        "private",
        "internal",
        "final",
        "open",
        "abstract",
        "sealed",
        "static",
        "pub",
        "pub(crate)",
        "data",
    ];

    pub const CSHARP_READABLE: [&str; 2] = ["public ", "internal "];

    pub const TYPE_FIRST: [(&str, bool); 10] = [
        ("public ", false),
        ("virtual ", false),
        ("override ", false),
        ("async ", false),
        ("required ", false),
        ("internal ", false),
        ("static ", false),
        ("final ", true),
        ("readonly ", true),
        ("const ", true),
    ];

    pub const CSHARP_TASKS: [(&str, &str); 4] = [
        (
            "System.Threading.Tasks.Task<BehaviorJSON>",
            "Task<BehaviorJSON>",
        ),
        ("Task<BehaviorJSON>", "Task<BehaviorJSON>"),
        ("System.Threading.Tasks.Task", "Task"),
        ("Task", "Task"),
    ];
}
