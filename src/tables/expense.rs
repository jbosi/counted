use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "expense")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub expense_id: i32,
    pub name: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub amount: f64,
	pub date: Date,
	pub paid_by_id: i32,
	pub paid_for_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::PaidById",
        to = "super::user::Column::Id"
    )]
	Usertest,
	#[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::PaidForId",
        to = "super::user::Column::Id"
    )]
	UserPaidFor
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usertest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
