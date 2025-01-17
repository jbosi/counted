
import { Component, OnInit, inject } from '@angular/core';
import { IExpensesViewModel, IUser, RouterParamService } from '@hcount/modules';
import { DialogService, DynamicDialogConfig, DynamicDialogModule } from 'primeng/dynamicdialog';
import { SubHeaderComponent } from '../../../../components';
import { ExpenseComponent } from './components';
import { AddExpenseModalComponent } from './components/add-project-expense';
import { ProjectApplication } from './project.application';
import { ButtonModule } from 'primeng/button';
import { RouterModule } from '@angular/router';

@Component({
    selector: 'app-project',
    templateUrl: './project.component.html',
    styleUrls: ['./project.component.scss'],
    imports: [
        ExpenseComponent,
        SubHeaderComponent,
        AddExpenseModalComponent,
        DynamicDialogModule,
        ButtonModule,
        RouterModule
    ],
    providers: [DialogService]
})
export class ProjectComponent implements OnInit {
	private readonly projectApplication = inject(ProjectApplication);
	private readonly routerParamService = inject(RouterParamService);
	private readonly dialogService = inject(DialogService);

	public users: IUser[] = [];
	public expensePayments: IExpensesViewModel[] = [];
	public projectId!: string;
	public globalTotal: number = 0;
	public expensePaymentId: number | undefined;
	
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

	public async onDisplayExpenseModal(users: IUser[], expensePaymentId?: number): Promise<void> {
		const config: DynamicDialogConfig<IExpenseModalData> = {
			data: {
				expensePayment: this.expensePayments?.find(ep => ep.id === expensePaymentId),
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