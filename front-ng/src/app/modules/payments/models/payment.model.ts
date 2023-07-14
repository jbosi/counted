export interface IPayment {
	id: number,
	expense_id: number,
	user_id: number,
	is_debt: boolean,
	amount: number,
	created_at: string
}