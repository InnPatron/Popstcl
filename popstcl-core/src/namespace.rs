#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Namespace {
    Local,
    Module,
    Args,
}
