import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Params } from '@angular/router';
import { IExpensePaymentsViewModel, IUser, RouterParamService } from '@hcount/modules';
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
		private readonly projectApplication: ProjectApplication,
		private readonly routerParamService: RouterParamService
	) {}
	
	async ngOnInit(): Promise<void> {
		const projectId = this.routerParamService.getParam('projectId');
		
		if (projectId == null) {
			return;
		}
		
		this.projectId = projectId;
		
		this.users = await this.projectApplication.getUsersByProjectIdAsync(this.projectId);

		this.expensePayments = await this.projectApplication.getExpensePaymentsAsync(this.projectId);
	}

	public async onExpenseAddedAsync(): Promise<void> {
		this.expensePayments = await this.projectApplication.getExpensePaymentsAsync(this.projectId);
	}
}
