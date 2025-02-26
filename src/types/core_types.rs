use serde::{Deserialize, Serialize};

//--------------primitive-----------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// represents a field that could be a single item or array of items
pub enum OptionalArray<T: Clone> {
    Single(T),
    Multiple(Vec<T>),
}

impl<T: Clone> OptionalArray<T> {
    pub fn into_array(self) -> Vec<T> {
        match self {
            OptionalArray::Single(x) => vec![x],
            OptionalArray::Multiple(items) => items,
        }
    }
}
