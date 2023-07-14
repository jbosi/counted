import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Params } from '@angular/router';
import { IExpensePaymentsViewModel, IUser } from '@hcount/modules';
import { firstValueFrom } from 'rxjs';
import { SubHeaderComponent } from '../../../../components';
import { ExpenseComponent } from './components';
import { AddExpenseModalComponent } from './components/add-project-expense';
import { ProjectApplication } from './project.application';

@Component({
	selector: 'app-project',
	templateUrl: './project.component.html',
	styleUrls: ['./project.component.scss'],
	standalone: true,
	imports: [
		ExpenseComponent,
		SubHeaderComponent,
		AddExpenseModalComponent,
		CommonModule,
	]
})
export class ProjectComponent implements OnInit {
	public users: IUser[] = [];
	public expensePayments: IExpensePaymentsViewModel[] = [];
	public projectId!: string;

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly projectApplication: ProjectApplication
	) {}
	
	async ngOnInit(): Promise<void> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		this.projectId = (params as { projectId: string }).projectId;
		
		this.users = await this.projectApplication.getUsersAsync();

		this.expensePayments = await this.projectApplication.getExpensePaymentsAsync(this.projectId);
	}

	public async onExpenseAddedAsync(): Promise<void> {
		this.expensePayments = await this.projectApplication.getExpensePaymentsAsync(this.projectId);
	}
}
