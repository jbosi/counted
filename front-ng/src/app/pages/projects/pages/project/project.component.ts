import { JsonPipe, NgFor } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Params } from '@angular/router';
import { firstValueFrom } from 'rxjs';
import { SubHeaderComponent } from '../../../../components';
import { ExpensesHttpClient, IExpense, IUser, UsersHttpClient } from '../../../../modules';
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
	public expenses: IExpense[] = [];

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly expensesHttpClient: ExpensesHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}
	
	async ngOnInit(): Promise<void> {
		this.expenses = await this.getExpensesAsync();
		this.users = await this.usersHttpClient.getAsync();
	}

	public async onExpenseAddedAsync(): Promise<void> {
		this.expenses = await this.getExpensesAsync();
	}

	private async getExpensesAsync(): Promise<IExpense[]> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		return await this.expensesHttpClient.getAsync((params as { projectId: string }).projectId);
	}
}
