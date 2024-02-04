import { ExpenseType } from "../../expenses";
import { IPaymentViewModel } from "../../payments";

export interface IExpensesViewModel {
	id: number,
	date: string,
	amount: number,
	name: string,
	expense_type: ExpenseType,
	payors: IPaymentViewModel[]
	debtors: IPaymentViewModel[]
	description?: string,
}