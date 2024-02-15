use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Self::create_table_ledger(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Ledger::Table).to_owned())
            .await


    }
}

impl Migration {
    async fn create_table_ledger(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ledger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ledger::PaymentId)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Ledger::AccountType)
                            .comment("Account Type of the Ledger Entry. Can be either 'Seller' or 'Buyer'")
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Ledger::DebitAmount)
                            .decimal()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Ledger::CreditAmount)
                            .decimal()
                            .not_null()
                            .default(0.0),
                    )
                    .col(
                        ColumnDef::new(Ledger::CreatedDateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        &mut Index::create()
                            .name("ledger_pk")
                            .col(Ledger::PaymentId)
                            .col(Ledger::AccountType)
                            .to_owned()
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Ledger {
    Table,
    PaymentId,
    AccountType,
    DebitAmount,
    CreditAmount,
    #[sea_orm(iden = "created_dt")]
    CreatedDateTime
}
