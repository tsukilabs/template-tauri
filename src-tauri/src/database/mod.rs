// pub mod entities;

pub mod prelude {
  // pub use super::entities::prelude::*;
  pub use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
}

// pub use entities::prelude::*;
// use migration::{Migrator, MigratorTrait};
use crate::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use tokio::fs;

pub fn connect(app: &AppHandle) -> Result<DatabaseConnection> {
  async_runtime::block_on(async move {
    let path = app.path().app_local_data_dir().unwrap();
    fs::create_dir_all(&path).await?;

    let path = path.join("database.sqlite");
    let url = format!("sqlite://{}?mode=rwc", path.to_str().unwrap());
    let conn = Database::connect(url).await?;

    // Migrator::up(&conn, None).await?;

    Ok(conn)
  })
}
