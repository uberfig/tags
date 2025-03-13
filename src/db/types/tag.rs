use crate::types::actors::Actor;

pub struct Tag {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub banned: bool,
}

impl Tag {
    pub fn to_actor(&self) -> Actor {
        todo!()
    }
}
