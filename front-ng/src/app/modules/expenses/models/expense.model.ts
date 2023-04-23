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

export interface ICreatableExpense {
	name: string,
	amount: number,
	expense_type: ExpenseType,
	
	payers: IUserAmount[],
	debtors: IUserAmount[],
	author_id: number,
	description?: string,
}

export interface IUserAmount {
	user_id: number,
	amount: number
}