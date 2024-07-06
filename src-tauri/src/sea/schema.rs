use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "word")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub word: String,
    pub gender: String,
    pub translation: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WordSet,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WordSet => Entity::has_many(super::word_set::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "set")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WordSet,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WordSet => Entity::has_many(super::word_set::Entity).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "word_set")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub word_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub set_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Word,
    Set,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Word => Entity::belongs_to(super::word::Entity)
                .from(Column::WordId)
                .to(super::word::Column::Id)
                .into(),
            Self::Set => Entity::belongs_to(super::set::Entity)
                .from(Column::SetId)
                .to(super::set::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}