import { JsonPipe, NgFor } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Params } from '@angular/router';
import { ExpensePaymentsHttpClient, IExpensePayments, IExpensePaymentsViewModel, IPaymentViewModel, IUser, UsersHttpClient } from '@hcount/modules';
import { firstValueFrom } from 'rxjs';
import { SubHeaderComponent } from '../../../../components';
import { ExpenseComponent } from './components';
import { AddExpenseModalComponent } from './components/add-project-expense';

@Component({
	selector: 'app-project',
	templateUrl: './project.component.html',
	styleUrls: ['./project.component.scss'],
	standalone: true,
	imports: [
		ExpenseComponent,
		SubHeaderComponent,
		AddExpenseModalComponent,
		JsonPipe,
		NgFor
	]
})
export class ProjectComponent implements OnInit {
	public users: IUser[] = [];
	public expensePayments: IExpensePaymentsViewModel[] = [];

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly expensePaymentsHttpClient: ExpensePaymentsHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}
	
	async ngOnInit(): Promise<void> {
		this.expensePayments = await this.getExpensePaymentsAsync();
	}

	public async onExpenseAddedAsync(): Promise<void> {
		this.expensePayments = await this.getExpensePaymentsAsync();
	}

	// should be in application
	private async getExpensePaymentsAsync(): Promise<IExpensePaymentsViewModel[]> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		const expensePayments: IExpensePayments[] = await this.expensePaymentsHttpClient.getAsync((params as { projectId: string }).projectId);
		this.users = await this.usersHttpClient.getAsync();
		return expensePayments.map(ep => {
			const payments: IPaymentViewModel[] = ep.payments
				.map(payment => {
					const user: IUser = this.users.find(u => u.id === payment.user_id) as IUser
					return {
						amount: payment.amount,
						id: payment.id,
						is_debt: payment.is_debt,
						user_name: user.name
					}
				})

			return {
				amount: ep.amount,
				date: ep.date,
				expense_type: ep.expense_type,
				id: ep.id,
				name: ep.name,
				description: ep.description,
				payors: payments.filter(payment => !payment.is_debt),
				debtors: payments.filter(payment => payment.is_debt)
			}
		})
	}
}
