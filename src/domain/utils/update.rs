#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Update<T> {
    #[default]
    Keep,
    Change(T),
}
