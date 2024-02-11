use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Ledger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ledger::PaymentId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Ledger::AccountType)
                            .comment("Account Type of the Ledger Entry. Can be either 'Seller' or 'Buyer'")
                            .string()
                            .not_null()
                            .primary_key(),
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
                    .to_owned(),
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Ledger::Table).to_owned())
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
}
