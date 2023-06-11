import { ExpenseType } from "../../expenses";
import { IPayment } from "../../payments";

export interface IExpensePayments {
	id: number,
	author_id: number,
	project_id: string,
	date: string,
	amount: number,
	name: string,
	expense_type: ExpenseType,
	payments: IPayment[]
	description?: string,
}