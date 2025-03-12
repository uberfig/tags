use crate::types::actors::Actor;

pub struct Tag {
    pub name: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

impl Tag {
    pub fn to_actor(&self) -> Actor {
        todo!()
    }
}
