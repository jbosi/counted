import { Injectable } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { ExpenseType, ExpensesHttpClient, ICreatableExpense, IExpenseDto } from '@hcount/modules';
import { IAddExpenseForm } from './add-expense-modal.component';

// TODO Rename into ExpenseModalApplication
@Injectable({ providedIn: 'root' })
export class AddExpenseModalApplication {
	constructor(
		private readonly expensesHttpClient: ExpensesHttpClient,
	) {}

	public async addExpenseModalAsync(form: FormGroup<IAddExpenseForm>, projectId: string): Promise<void> {
		const formValues = form.getRawValue();
		const expenseTypeValue = formValues?.expenseType?.[0];

		const candidate: ICreatableExpense = {
			name: formValues.name as string,
			amount: formValues.amount,
			expense_type: typeof expenseTypeValue === 'string' ? expenseTypeValue : expenseTypeValue?.id, // Initial value is not an object
			debtors: formValues.debtors.map(d => ({ amount: d.userAmount, user_id: d.userAmount })),
			payers: formValues.payers.map(d => ({ amount: d.userAmount, user_id: d.userAmount })),
			project_id: projectId,
			description: formValues?.description ?? undefined,
			author_id: 1, // TODO set in backend
		}

		await this.expensesHttpClient.createAsync(candidate)
			.catch(e => console.error(e));
	}

	public async getExpenseByIdAsync(expenseId: number): Promise<IExpenseDto | null> {
		return this.expensesHttpClient.getByIdAsync(expenseId)
			.catch(() => null);
	}

	public async deleteExpenseAsync(expenseId: number): Promise<void> {
		await this.expensesHttpClient.deleteAsync(expenseId);
	}
}