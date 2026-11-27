pub const CONFIGURE: &'static str = "    private static BehaviorAdapters Configure(";
pub const CONFIGURATIONS_FIELD: &'static str = "    public static System.Func<System.Type, BehaviorExecution, BehaviorAdapters> Configurations;\n";
pub const CONFIGURATIONS_CALL: &'static str = "if (Configurations != null) { var adapters = Configurations(configuration, execution); if (adapters != null) return adapters; }";
pub const WRITTEN_BODIES: [&'static str; 3] = ["gateway", "query", "mapping"];
pub const CONTEXT: &'static str = "behaviors";
pub const TAC: &'static str = "tac";
pub const NO_STORAGE: &'static str = "";
pub const NO_REGISTRY: &'static str = "{}";
pub const PAGE_STORAGE: &'static str = " BehaviorStorage.Host ??= TacWeb.Storage;";
