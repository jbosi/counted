import { Injectable } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { ExpenseType, ExpensesHttpClient, ICreatableExpense } from '@hcount/modules';
import { IAddExpenseForm } from './add-expense-modal.component';

@Injectable({ providedIn: 'root' })
export class AddExpenseModalApplication {
	constructor(
		private readonly expensesHttpClient: ExpensesHttpClient,
	) {}

	public async addExpenseModalAsync(form: FormGroup<IAddExpenseForm>, projectId: string): Promise<void> {
		const formValues = form.getRawValue();

		const candidate: ICreatableExpense = {
			name: formValues.name as string,
			amount: formValues.amount,
			expense_type: formValues?.expenseType?.[0] as ExpenseType,
			debtors: formValues.debtors.map(d => ({ amount: d.userAmount, user_id: d.userAmount })),
			payers: formValues.payers.map(d => ({ amount: d.userAmount, user_id: d.userAmount })),
			project_id: projectId,
			description: formValues?.description,
			author_id: 1, // TODO set in backend
		}

		await this.expensesHttpClient.createAsync(candidate)
			.catch(e => console.error(e));
	}

	public async deleteExpenseAsync(expenseId: number): Promise<void> {
		await this.expensesHttpClient.deleteAsync(expenseId);
	}
}