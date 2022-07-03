#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub balance: f64,
    pub published: bool
}

#[derive(Queryable)]
pub struct Expenses {
    pub id: i32,
    pub paid_for_id: i32,
    pub paid_by_id: i32,
    pub author_id: i32,
    pub project_id: i32,
    pub date: Date,
    pub amount: f64,
    pub description: String,
    pub name: String,
    pub expense_type: String
}

#[derive(Queryable)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub created_at: Date,
    pub total_expenses: f64,
    pub currency: String
}
