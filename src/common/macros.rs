#[macro_export]
macro_rules! define_repo_delete_fn {
    ($fn_name:ident, $table_name:expr) => {
        pub async fn $fn_name(
            state: &crate::db::AppState,
            id: uuid::Uuid,
        ) -> Result<(), crate::error::ApiError> {
            let sql = concat!("DELETE FROM ", $table_name, " WHERE id = $1");
            sqlx::query(sql)
                .bind(id)
                .execute(crate::db::db_conn(state))
                .await
                .map_err(|e| {
                    tracing::error!(
                        concat!(stringify!($fn_name), " failed: {}\nSQL: {}"),
                        e,
                        sql
                    );
                    crate::error::ApiError::DatabaseError(e)
                })?;
            Ok(())
        }
    };
}
