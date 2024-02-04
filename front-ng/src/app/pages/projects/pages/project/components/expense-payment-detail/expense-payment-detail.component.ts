import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { RouterModule } from '@angular/router';
import { AvatarInitialsComponent, IExpensesViewModel, RouterParamService } from '@hcount/modules';
import { ProjectApplication } from '../../project.application';

@Component({
	selector: 'app-expense-payment-detail',
	templateUrl: './expense-payment-detail.component.html',
	styleUrls: ['./expense-payment-detail.component.scss'],
	standalone: true,
	imports: [AvatarInitialsComponent, CommonModule, RouterModule],
	changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExpensePaymentDetailComponent implements OnInit {
	public expensePayment!: IExpensesViewModel | undefined;

	constructor(
		private readonly projectApplication: ProjectApplication,
		private readonly routerParamService: RouterParamService,
		private readonly cdr: ChangeDetectorRef
	) {}
	
	async ngOnInit(): Promise<void> {
		const expensePaymentId = this.routerParamService.getParsedParam('expensePaymentId');
		const projectId = this.routerParamService.getParam('projectId');

		if (expensePaymentId == null || projectId == null) {
			console.error(`Route parameter error, could not retrieve ${projectId} and ${expensePaymentId}`);
			return;
		}
		
		this.expensePayment = await this.projectApplication.getExpenseAsync(projectId, expensePaymentId);
		this.cdr.markForCheck();
	}
}