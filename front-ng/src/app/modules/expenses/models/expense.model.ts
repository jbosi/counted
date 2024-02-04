import { IPayment } from "../../payments"

export interface IExpenseDto {
	id: number,
	author_id: number,
	project_id: string,
	date: string,
	amount: number,
	name: string,
	expense_type: ExpenseType,
	payments: IPayment[],
	description?: string,
}

export enum ExpenseType {
	Expense = 'Expense',
	Transfer = 'Transfer',
	Gain = 'Gain'
}

export interface ICreatableExpense {
	name: string,
	amount: number,
	expense_type: ExpenseType,
	project_id: string;
	payers: IUserAmount[],
	debtors: IUserAmount[],
	author_id: number,
	description?: string,
}

export interface IUserAmount {
	user_id: number,
	amount: number
}