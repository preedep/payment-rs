use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ledgers")]
pub struct Model {
    //#[sea_orm(primary_key, column_name = "payment_id")]
    #[sea_orm(primary_key)]
    pub payment_id: String,
    #[sea_orm(primary_key,column_name = "account_type")]
    pub account_type : String,
    #[sea_orm(column_name = "debit_amount" , default_value = 0.0)]
    pub debit_amount: f64,
    #[sea_orm(column_name = "credit_amount" , default_value = 0.0)]
    pub credit_amount: f64
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
