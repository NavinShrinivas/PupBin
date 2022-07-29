use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Paste::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Paste::UrlHash)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Paste::PasteContent).string_len(10475760).not_null()) //10 MB limit
                    .col(ColumnDef::new(Paste::DeletionDate).date_time_len(4).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Paste::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Paste {
    Table,
    UrlHash,
    PasteContent,
    DeletionDate,
}
