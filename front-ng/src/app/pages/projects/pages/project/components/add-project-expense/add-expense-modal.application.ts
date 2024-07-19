import { Injectable } from '@angular/core';
import { FormGroup } from '@angular/forms';
import { ExpensesHttpClient, ICreatableExpense, IEditableExpense, IExpenseDto } from '@hcount/modules';
import { IAddExpenseForm } from './add-expense-modal.component';

// TODO Rename into ExpenseModalApplication
@Injectable({ providedIn: 'root' })
export class AddExpenseModalApplication {
	constructor(
		private readonly expensesHttpClient: ExpensesHttpClient,
	) {}

	public async createOrEditExpenseAsync(form: FormGroup<IAddExpenseForm>, projectId: string, expenseId: number | undefined): Promise<void> {
		const formValues = form.getRawValue();
		const expenseTypeValue = formValues?.expenseType?.[0];

		const candidate: ICreatableExpense = {
			name: formValues.name as string,
			amount: parseFloat(formValues.amount as unknown as string),
			expense_type: typeof expenseTypeValue === 'string' ? expenseTypeValue : expenseTypeValue?.id, // Initial value is not an object
			debtors: formValues.debtors.map(d => ({ amount: d.userAmount, user_id: d.userId })),
			payers: formValues.payers.map(d => ({ amount: d.userAmount, user_id: d.userId })),
			project_id: projectId,
			description: formValues?.description ?? undefined,
			author_id: 1, // TODO set in backend
		}

		if (!!expenseId) {
			const editableCandidate: IEditableExpense = {
				...candidate,
				id: expenseId
			}
			
			await this.expensesHttpClient.editAsync(editableCandidate, expenseId)
				.catch(e => console.error(e));
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