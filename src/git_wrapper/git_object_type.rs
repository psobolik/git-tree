#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GitObjectType {
    Commit,
    Blob,
    Tree,
}
