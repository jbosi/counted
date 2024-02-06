import { Inject, Injectable } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { ExpensesHttpClient, ICreatableExpense, IExpenseDto, IExpensePayments, IExpensesViewModel, IPaymentViewModel, IPrincipal, IUser, PRINCIPAL, UsersHttpClient } from '@hcount/modules';
import { IAddExpenseForm } from './add-expense-modal.component';

@Injectable({ providedIn: 'root' })
export class AddExpenseModalApplication {
	constructor(
		private readonly expensesHttpClient: ExpensesHttpClient,
	) {}

	public async addExpenseModalAsync(form: FormGroup<IAddExpenseForm>, projectId: string): Promise<void> {
		const amount: number = form.value?.amount as number;
		const payers: number[] = form?.value?.payers as number[];
		const debtors: number[] = form?.value?.debtors as number[];
		const candidate: ICreatableExpense = {
			name: form?.value?.name,
			amount: amount,
			expense_type: form?.value?.expenseType?.[0],
			debtors: debtors?.map(id => ({ amount: amount / (debtors.length), user_id: id })),
			payers: payers?.map(id => ({ amount: amount / (payers.length), user_id: id })),
			project_id: projectId,
			description: form?.value?.description,
			author_id: 1, // TODO set in backend
		} as ICreatableExpense

		await this.expensesHttpClient.createAsync(candidate)
			.catch(e => console.error(e));
	}
}