use crate::{
    cryptography::key::{Algorithms, PrivateKey},
    db::pg_sesh::Sesh, protocol::{ap_protocol::fetch::authorized_fetch, errors::FetchErr, webfinger_resolve::webfinger_resolve}, types::actors::Actor,
};
use deadpool_postgres::Pool;
use url::Url;

use super::types::{instance_actor::InstanceActor, tag::Tag, user::User};

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

impl PgConn {
    /// gets the instance actor if exists or creates a new
    pub async fn get_or_init_instance_actor(&self) -> InstanceActor {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        if let Some(actor) = sesh.fetch_instance_actor().await {
            return actor;
        }
        //init the actor
        let new_actor = InstanceActor::new(Algorithms::RsaSha256);
        sesh.insert_instance_actor(&new_actor)
            .await
            .expect("failed to insert new instance actor");
        sesh.commit().await;
        new_actor
    }
    pub async fn get_instance_actor(&self) -> Option<InstanceActor> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.fetch_instance_actor().await
    }
    pub async fn get_or_init_tag(&self, tag: &str, banned: bool) -> Tag {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        if let Some(tag) = sesh.get_tag(tag).await {
            return tag;
        }
        let tag = sesh.create_tag(tag, banned).await;
        sesh.commit().await;
        tag
    }
    /// backfills users if they are not already present in the db
    pub async fn get_or_init_user<T: PrivateKey>(
        &self,
        username: &str,
        domain: &Url,
        private_key: &mut T,
    ) -> Result<User, FetchErr> {
        let mut client = self.db.get().await.expect("failed to get client");
        let transaction = client
            .transaction()
            .await
            .expect("failed to begin transaction");
        let sesh = Sesh::Transaction(transaction);
        if let Some(user) = sesh.get_user(username, domain).await {
            return Ok(user);
        }
        // ---------------- backfill ------------------
        let resolve = webfinger_resolve(username, domain.as_str()).await;
        let resolve = match resolve {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        };
        let Some(resolve) = resolve.get_self() else {
            return Err(FetchErr::MissingField("Self".to_string()));
        };

        let fetched: Result<Actor, FetchErr> = authorized_fetch(
            signature.signature_header.key_id.clone(),
            instance_key_id,
            instance_private_key,
        )
        .await;

        sesh.commit().await;

        todo!()
    }
}
