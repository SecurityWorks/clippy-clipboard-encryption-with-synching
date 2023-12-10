//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "clipboard"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub r#type: String,
    pub content: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub size: Option<String>,
    pub blob: Option<Vec<u8>>,
    pub base64: Option<String>,
    pub star: Option<bool>,
    pub created_date: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Type,
    Content,
    Width,
    Height,
    Size,
    Blob,
    Base64,
    Star,
    CreatedDate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::Integer.def(),
            Self::Type => ColumnType::String(None).def(),
            Self::Content => ColumnType::String(None).def().null(),
            Self::Width => ColumnType::Integer.def().null(),
            Self::Height => ColumnType::Integer.def().null(),
            Self::Size => ColumnType::String(None).def().null(),
            Self::Blob => ColumnType::Binary(BlobSize::Blob(None)).def().null(),
            Self::Base64 => ColumnType::String(None).def().null(),
            Self::Star => ColumnType::Boolean.def().null(),
            Self::CreatedDate => ColumnType::String(None).def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
