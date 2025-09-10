#[derive(Debug)]
#[allow(unused)]
pub enum TokenNodeChange {
    Remove,
    AddLeft(String),
    AddRight(String),
    ReplaceWith(String),
}
