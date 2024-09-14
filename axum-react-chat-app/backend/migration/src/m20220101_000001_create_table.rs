use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Room::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Room::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Room::Participants)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chat::Timestamp).timestamp().not_null())
                    .col(ColumnDef::new(Chat::Sender).string().not_null())
                    .col(ColumnDef::new(Chat::Message).string().not_null())
                    .col(ColumnDef::new(Chat::RoomId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_room_id")
                            .from_tbl(Chat::Table)
                            .from_col(Chat::RoomId)
                            .to_tbl(Room::Table)
                            .to_col(Room::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Room::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Chat::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
}

#[derive(DeriveIden)]
enum Room {
    Table,
    Id,
    Participants,
}

#[derive(DeriveIden)]
enum Chat {
    Table,
    Id,
    Timestamp,
    Sender,
    Message,
    RoomId,
}
