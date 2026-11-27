use crate::behavior_module::Module;

pub enum Prepared {
    Erased(String),
    Annotated(Module),
}
