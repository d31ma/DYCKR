#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriterMode {
    Top,
    Table,
    Parameters,
    Body,
}
