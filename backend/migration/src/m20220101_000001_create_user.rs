use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Column::Username).string().not_null())
                    .col(ColumnDef::new(Column::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned()).await
    }
}

#[derive(Iden)]
enum Entity {
    Users,
}

#[derive(Iden)]
enum Column {
    Id,
    Username,
    Email,
}