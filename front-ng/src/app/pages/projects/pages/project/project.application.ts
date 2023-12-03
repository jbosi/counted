import { Injectable } from '@angular/core';
import { ExpensePaymentsHttpClient, IExpensePayments, IExpensePaymentsViewModel, IPaymentViewModel, IUser, UsersHttpClient } from '@hcount/modules';

@Injectable({ providedIn: 'root' })
export class ProjectApplication {
	constructor(
		private readonly expensePaymentsHttpClient: ExpensePaymentsHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}

	public async getUsersAsync(): Promise<IUser[]> {
		return this.usersHttpClient.getAsync();
	}

	public async getUsersByProjectIdAsync(projectId: string): Promise<IUser[]> {
		return this.usersHttpClient.getUsersByProjectIdAsync(projectId);
	}

	public async getExpensePaymentsAsync(projectId: string): Promise<IExpensePaymentsViewModel[]> {
		const expensePayments: IExpensePayments[] = await this.expensePaymentsHttpClient.getAsync(projectId);
		const users = await this.getUsersByProjectIdAsync(projectId);

		return expensePayments.map(ep => this.forgeExpensePaymentsViewModel(ep, users));
	}

	public async getExpensePaymentAsync(projectId: string, expensePaymentId: number): Promise<IExpensePaymentsViewModel | undefined> {
		const expensePayments: IExpensePayments | undefined = await this.expensePaymentsHttpClient.getByIdAsync(projectId, expensePaymentId);

		if (expensePayments == null) {
			return;
		}

		const users = await this.getUsersByProjectIdAsync(projectId);
		
		return this.forgeExpensePaymentsViewModel(expensePayments, users);
	}


	private forgeExpensePaymentsViewModel(expensePayment: IExpensePayments, users: IUser[]): IExpensePaymentsViewModel {
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