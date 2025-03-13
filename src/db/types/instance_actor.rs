use url::Url;

use crate::{cryptography::key::Algorithms, types::actors::Actor};

pub struct InstanceActor {
    pub private_key_pem: String,
    pub public_key_pem: String,
    pub algorithm: Algorithms,
}

impl InstanceActor {
    pub fn pub_key_id(domain: &str) -> String {
        format!("https://{domain}/actor/ap#main-key")
    }
    pub fn get_key_id(domain: &str) -> String {
        format!("https://{}/{}", domain, domain)
    }
    pub fn activitypub_id(_domain: &str) -> Url {
        todo!()
    }
    pub fn to_actor(&self, _domain: &str) -> Actor {
        //     // let links = instance_actor_links(domain);
        //     todo!()
        //     // Actor {
        //     //     type_field: crate::activitystream_objects::actors::ActorType::Application,
        //     //     id: links.id.clone(),
        //     //     preferred_username: domain.to_string(),
        //     //     summary: None,
        //     //     name: None,
        //     //     url: Some(
        //     //         Url::parse(&format!("https://{domain}/about/more?instance_actor=true")).unwrap(),
        //     //     ),
        //     //     public_key: PublicKey {
        //     //         id: links.pub_key_id,
        //     //         owner: links.id,
        //     //         public_key_pem: self.public_key_pem.clone(),
        //     //     },
        //     //     inbox: links.inbox,
        //     //     outbox: links.outbox,
        //     //     followers: links.followers,
        //     //     following: links.following,
        //     //     domain: Some(domain.to_string()),
        //     //     liked: Some(links.liked),
        //     // }
        todo!()
    }
    pub fn new(algo: Algorithms) -> Self {
        todo!()
    }
}
