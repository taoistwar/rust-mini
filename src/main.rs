use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use rust_mini::DatabaseConfig;
use tokio_postgres::NoTls;

include!(concat!(env!("OUT_DIR"), "/controller-sql.rs"));

#[tokio::main]
async fn main() {
    let config = DatabaseConfig::load();
    let mut cfg = deadpool_postgres::Config::new();
    cfg.dbname = Some(config.name);
    cfg.host = Some(config.host);
    cfg.user = Some(config.user);
    cfg.password = Some(config.password);
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let pool = cfg
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
        .unwrap();
    let client = pool.get().await.unwrap();
    let org = "org".to_string();
    let res = queries::api_queries::get_sources()
        .bind(&client, &org)
        .all()
        .await
        .unwrap();
    res.into_iter().for_each(|rec| {
        println!("{:?}", rec);
    });
}
