pub struct GatewayLimits;

impl GatewayLimits {
    pub const MAX_ARGUMENTS: usize = 32;
    pub const MAX_BYTES: usize = 1024 * 1024;
    pub const MAX_DEPTH: usize = 32;
    pub const MAX_NODES: usize = 10_000;
    pub const MAX_CALLS: usize = 64;
    pub const MAX_COMMAND_ENTRIES: usize = 32;
    pub const MAX_ENTRY_BYTES: usize = 4096;
    pub const MAX_CLASS_PATH_BYTES: usize = 1024;
}
