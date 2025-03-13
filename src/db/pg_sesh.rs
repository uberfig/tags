use deadpool_postgres::{Object, Transaction};
use tokio_postgres::{types::ToSql, Statement};

use crate::cryptography::key::Algorithms;

use super::types::{instance_actor::InstanceActor, tag::Tag};

pub enum Sesh<'a> {
    Client(Object),
    Transaction(Transaction<'a>),
}
impl Sesh<'_> {
    pub async fn commit(self) {
        if let Sesh::Transaction(transaction) = self {
            transaction.commit().await.expect("failed to commit")
        }
    }
    pub async fn query(
        &self,
        stmt: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        let stmt = self.prepare(stmt).await;
        self.query_stmt(&stmt, params).await
    }
    pub async fn prepare(&self, stmt: &str) -> Statement {
        match self {
            Sesh::Client(object) => object.prepare(stmt).await.expect("failed to prepare query"),
            Sesh::Transaction(transaction) => transaction
                .prepare(stmt)
                .await
                .expect("failed to prepare query"),
        }
    }
    pub async fn query_stmt(
        &self,
        stmt: &Statement,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        match self {
            Sesh::Client(object) => object.query(stmt, params).await,
            Sesh::Transaction(transaction) => transaction.query(stmt, params).await,
        }
    }
}

// instance actor
impl Sesh<'_> {
    pub async fn fetch_instance_actor(&self) -> Option<InstanceActor> {
        let stmt = r#"
            SELECT * FROM ap_instance_actor LIMIT 1;
        "#;

        let result = self
            .query(stmt, &[])
            .await
            .expect("failed to get instance actor")
            .pop();

        match result {
            Some(result) => {
                let algorithm: String = result.get("algorithm");
                Some(InstanceActor {
                    private_key_pem: result.get("private_key_pem"),
                    public_key_pem: result.get("public_key_pem"),
                    algorithm: Algorithms::try_from(algorithm.as_str())
                        .expect("unkown algorithm in db"),
                })
            }
            None => None,
        }
    }
    pub async fn insert_instance_actor(
        &self,
        instance_actor: &InstanceActor,
    ) -> Result<(), String> {
        let stmt = r#"
        INSERT INTO ap_instance_actor
        (private_key_pem, public_key_pem, algorithm)
        VALUES
        ($1, $2, $3);
        "#;

        let result = self
            .query(
                stmt,
                &[
                    &instance_actor.private_key_pem,
                    &instance_actor.public_key_pem,
                    &instance_actor.algorithm.to_string(),
                ],
            )
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

fn tag_from_row(row: tokio_postgres::Row) -> Tag {
    Tag {
        id: row.get("tag_id"),
        name: row.get("tag"),
        display_name: row.get("display_name"),
        bio: row.get("bio"),
        banned: row.get("banned")
    }
}

// tags
impl Sesh<'_> {
    pub async fn get_tag(&self, tag: &str) -> Option<Tag> {
        let stmt = r#"
            SELECT * FROM tags WHERE tag = $1;
        "#;
        let result = self
            .query(stmt, &[&tag])
            .await
            .expect("failed to fetch tag")
            .pop();
        match result {
            Some(row) => {
                Some(tag_from_row(row))
            },
            None => None,
        }
    }
    pub async fn set_tag_banned(&self, tag_id: i64, banned: bool) -> Result<(), ()> {
        let stmt = r#"
            UPSATE tags SET banned = $1 WHERE tag_id = $2;
        "#;
        let result = self
            .query(stmt, &[&banned, &tag_id])
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
    pub async fn create_tag(&self, tag: &str, banned: bool) -> Tag {
        let stmt = r#"
        INSERT INTO tags
        (tag, banned)
        VALUES
        ($1, $2)
        RETURNING *;
        "#;
        let result = self
            .query(stmt, &[&tag, &banned])
            .await
            .expect("failed to create tag")
            .pop()
            .expect("creating tag returned nothing");
        tag_from_row(result)
    }
    pub async fn update_tag(&self, tag: &Tag) -> Tag {
        let stmt = r#"
        UPDATE tags SET
        display_name = $1,
        bio = $2,
        banned = $3
        WHERE tag_id = $4
        RETURNING *;
        "#;
        let result = self
            .query(stmt, &[&tag.display_name, &tag.bio, &tag.banned, &tag.id])
            .await
            .expect("failed to update tag")
            .pop()
            .expect("updating tag returned nothing");
        tag_from_row(result)
    }
}

