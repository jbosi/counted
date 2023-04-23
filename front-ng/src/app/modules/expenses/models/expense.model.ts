export interface IExpense {
	id: number,
	author_id: number,
	project_id: string,
	date: string,
	amount: number,
	description?: string,
	name: string,
	expense_type: ExpenseType,
}

export enum ExpenseType {
	Expense,
	Transfer,
	Gain
}