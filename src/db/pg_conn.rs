use deadpool_postgres::{Object, Pool, Transaction};
use tokio_postgres::{types::ToSql, Statement};

use crate::types::actors::Actor;

use super::types::instance_actor::InstanceActor;

#[derive(Clone, Debug)]
pub struct PgConn {
    pub db: Pool,
}

enum Sesh<'a> {
    Client(Object),
    Transaction(Transaction<'a>),
}
impl Sesh<'_> {
    async fn query(
        &self,
        stmt: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> {
        let stmt = self.prepare(stmt).await;
        self.query_stmt(&stmt, params).await
    }
    async fn prepare(&self, stmt: &str) -> Statement {
        match self {
            Sesh::Client(object) => object.prepare(stmt).await.expect("failed to prepare query"),
            Sesh::Transaction(transaction) => transaction
                .prepare(stmt)
                .await
                .expect("failed to prepare query"),
        }
    }
    async fn query_stmt(
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

impl Sesh<'_> {
    async fn fetch_instance_actor(&self) -> Option<InstanceActor> {
        let stmt = r#"
            SELECT * FROM ap_instance_actor LIMIT 1;
        "#;

        let result = self
            .query(stmt, &[])
            .await
            .expect("failed to get instance actor")
            .pop();

        match result {
            Some(result) => Some(InstanceActor {
                private_key_pem: result.get("private_key_pem"),
                public_key_pem: result.get("public_key_pem"),
            }),
            None => None,
        }
    }
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
        todo!()
    }
    pub async fn get_instance_actor(&self) -> Option<InstanceActor> {
        let client = self.db.get().await.expect("failed to get client");
        let sesh = Sesh::Client(client);
        sesh.fetch_instance_actor().await
    }
    pub async fn get_or_init_tag(&self, tag: &str) -> Actor {
        todo!()
    }
}
