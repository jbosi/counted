import { JsonPipe } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Params } from '@angular/router';
import { firstValueFrom } from 'rxjs';
import { ExpensesHttpClient, IExpense } from 'src/app/modules/expenses';
import { SubHeaderComponent } from '../../../../components';
import { ExpenseComponent } from './components/expense/expense.component';

@Component({
	selector: 'app-project',
	templateUrl: './project.component.html',
	styleUrls: ['./project.component.scss'],
	standalone: true,
	imports: [
		ExpenseComponent,
		SubHeaderComponent,
		JsonPipe
	]
})
export class ProjectComponent implements OnInit {
	public expenses: IExpense[] = [];

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly expensesHttpClient: ExpensesHttpClient
	) {}
	
	// ngOnInit(): void {
	// 	this.activatedRoute.params.subscribe((p) => {
	// 		const value = p as { projectId: number };
	// 	})
	// }
	
	async ngOnInit(): Promise<void> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		this.expenses = await this.expensesHttpClient.getAsync((params as { projectId: string }).projectId);
	}
}