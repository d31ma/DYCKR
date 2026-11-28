pub const CONFIGURE: &str = "    private static BehaviorAdapters Configure(";
pub const CONFIGURATIONS_FIELD: &str = "    public static System.Func<System.Type, BehaviorExecution, BehaviorAdapters> Configurations;\n";
pub const CONFIGURATIONS_CALL: &str = "if (Configurations != null) { var adapters = Configurations(configuration, execution); if (adapters != null) return adapters; }";
pub const WRITTEN_BODIES: [&str; 3] = ["gateway", "query", "mapping"];
pub const CONTEXT: &str = "behaviors";
pub const TAC: &str = "tac";
pub const NO_STORAGE: &str = "";
pub const NO_REGISTRY: &str = "{}";
pub const PAGE_STORAGE: &str = " BehaviorStorage.Host ??= TacWeb.Storage;";
