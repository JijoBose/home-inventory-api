use sea_orm_migration::prelude::*;


use crate::m20220101_000001_create_house::House;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Room::Name).string().not_null())
                    .col(ColumnDef::new(Room::HouseId).string().not_null())
                    .foreign_key(
                      ForeignKey::create()
                        .name("fk-room-house_id")
                        .from(Room::Table, Room::Id)
                        .to(House::Table, House::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Room::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Room {
    Table,
    Id,
    Name,
    HouseId,
}
