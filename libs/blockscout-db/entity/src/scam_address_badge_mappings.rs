//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "scam_address_badge_mappings")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "VarBinary(StringLen::None)"
    )]
    pub address_hash: Vec<u8>,
    pub inserted_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::addresses::Entity",
        from = "Column::AddressHash",
        to = "super::addresses::Column::Hash",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Addresses,
}

impl Related<super::addresses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addresses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
