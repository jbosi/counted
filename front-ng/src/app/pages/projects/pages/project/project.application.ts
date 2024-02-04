import { Injectable } from '@angular/core';
import { ExpensesHttpClient, IExpenseDto, IExpensePayments, IExpensesViewModel, IPaymentViewModel, IUser, UsersHttpClient } from '@hcount/modules';

@Injectable({ providedIn: 'root' })
export class ProjectApplication {
	constructor(
		private readonly expensesHttpClient: ExpensesHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}

	public async getUsersAsync(): Promise<IUser[]> {
		return this.usersHttpClient.getAsync();
	}

	public async getUsersByProjectIdAsync(projectId: string): Promise<IUser[]> {
		return this.usersHttpClient.getUsersByProjectIdAsync(projectId);
	}

	public async getExpensesAsync(projectId: string): Promise<IExpensesViewModel[]> {
		const expensePayments: IExpenseDto[] = await this.expensesHttpClient.getAsync(projectId);
		const users = await this.getUsersByProjectIdAsync(projectId);

		return expensePayments.map(ep => this.forgeExpensePaymentsViewModel(ep, users));
	}

	public async getExpenseAsync(projectId: string, expenseId: number): Promise<IExpensesViewModel | undefined> {
		const expensePayments: IExpenseDto | undefined = await this.expensesHttpClient.getByIdAsync(expenseId);

		if (expensePayments == null) {
			return;
		}

		const users = await this.getUsersByProjectIdAsync(projectId);
		
		return this.forgeExpensePaymentsViewModel(expensePayments, users);
	}


	private forgeExpensePaymentsViewModel(expensePayment: IExpensePayments, users: IUser[]): IExpensesViewModel {
		const payments: IPaymentViewModel[] = expensePayment.payments
				.map(payment => {
					const user: IUser | undefined = users.find(u => u.id === payment.user_id)
					return {
						amount: payment.amount,
						id: payment.id,
						is_debt: payment.is_debt,
						user_name: user?.name ?? 'Error while fetching user'
					}
				})

			return {
				amount: expensePayment.amount,	
				date: expensePayment.date,
				expense_type: expensePayment.expense_type,
				id: expensePayment.id,
				name: expensePayment.name,
				description: expensePayment.description,
				payors: payments.filter(payment => !payment.is_debt),
				debtors: payments.filter(payment => payment.is_debt)
			}
	}
}