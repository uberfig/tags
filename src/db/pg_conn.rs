use crate::{cryptography::key::Algorithms, db::pg_sesh::Sesh};
use deadpool_postgres::Pool;
use url::Url;

use super::types::{instance_actor::InstanceActor, tag::Tag};

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
    pub async fn get_or_init_user(&self, username: &str, domain: &Url) {
        
    }
}
