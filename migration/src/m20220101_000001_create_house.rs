use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(House::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(House::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(House::Title).string().not_null())
                    .col(ColumnDef::new(House::Body).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(House::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum House {
    Table,
    Id,
    Title,
    Body,
}
