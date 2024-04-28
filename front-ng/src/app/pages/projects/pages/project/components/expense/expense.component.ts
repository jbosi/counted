import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { RouterModule } from '@angular/router';
import { AvatarInitialsComponent, IExpensesViewModel } from '@hcount/modules';
import { AvatarModule } from 'primeng/avatar';
@Component({
	selector: 'app-expense',
	templateUrl: './expense.component.html',
	styleUrls: ['./expense.component.scss'],
	standalone: true,
	imports: [AvatarModule, CommonModule, RouterModule, AvatarInitialsComponent],
	changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExpenseComponent {
	@Input() public expensePayment!: IExpensesViewModel;
	@Output() public displayExpenseDetail = new EventEmitter<number>();
}