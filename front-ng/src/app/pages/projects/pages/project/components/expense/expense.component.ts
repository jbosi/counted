import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { IExpensePaymentsViewModel } from '@hcount/modules';
import { AvatarModule } from 'primeng/avatar';

@Component({
	selector: 'app-expense',
	templateUrl: './expense.component.html',
	styleUrls: ['./expense.component.scss'],
	standalone: true,
	imports: [AvatarModule, CommonModule],
	changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExpenseComponent {
	@Input() public expensePayment!: IExpensePaymentsViewModel;

	constructor() {}
}