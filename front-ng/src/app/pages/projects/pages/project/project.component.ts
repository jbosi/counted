import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { IExpensesViewModel, IUser, RouterParamService } from '@hcount/modules';
import { DialogService, DynamicDialogConfig, DynamicDialogModule } from 'primeng/dynamicdialog';
import { SubHeaderComponent } from '../../../../components';
import { ExpenseComponent } from './components';
import { AddExpenseModalComponent } from './components/add-project-expense';
import { ProjectApplication } from './project.application';
import { ButtonModule } from 'primeng/button';

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
		DynamicDialogModule,
		ButtonModule
	],
	providers: [DialogService]
})
export class ProjectComponent implements OnInit {
	public users: IUser[] = [];
	public expensePayments: IExpensesViewModel[] = [];
	public projectId!: string;
	public globalTotal: number = 0;
	public expensePaymentId: number | undefined;

	constructor(
		private readonly projectApplication: ProjectApplication,
		private readonly routerParamService: RouterParamService,
		private readonly dialogService: DialogService
	) {}
	
	async ngOnInit(): Promise<void> {
		const projectId = this.routerParamService.getParam('projectId');
		
		if (projectId == null) {
			return;
		}
		
		this.projectId = projectId;
		
		this.users = await this.projectApplication.getUsersByProjectIdAsync(this.projectId);

		this.expensePayments = await this.projectApplication.getExpensesAsync(this.projectId);

		// TODO should be computed and use from backend
		this.globalTotal = this.expensePayments.map(ep => ep.amount).reduce((acc, ep) => acc + ep, 0);
	}

	public async onDisplayExpenseModal(users: IUser[], expensePayment?: IExpensesViewModel): Promise<void> {
		const config: DynamicDialogConfig<IExpenseModalData> = {
			data: {
				expensePayment,
				users: users
			},
			header: 'Créer une nouvelle dépense',
			width: '50vw',
			height: '600px'
		}
		this.dialogService.open(AddExpenseModalComponent, config).onClose.subscribe(async expense => {
			if (expense != null) {
				this.expensePayments = await this.projectApplication.getExpensesAsync(this.projectId);
			}
		});
	}
}

export interface IExpenseModalData {
	expensePayment?: IExpensesViewModel,
	users: IUser[]
}