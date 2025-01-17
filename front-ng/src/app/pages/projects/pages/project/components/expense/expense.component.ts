
import { ChangeDetectionStrategy, Component, EventEmitter, Output, input } from '@angular/core';
import { RouterModule } from '@angular/router';
import { AvatarInitialsComponent, IExpensesViewModel } from '@hcount/modules';
import { AvatarModule } from 'primeng/avatar';
@Component({
    selector: 'app-expense',
    templateUrl: './expense.component.html',
    styleUrls: ['./expense.component.scss'],
    imports: [AvatarModule, RouterModule, AvatarInitialsComponent],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExpenseComponent {
	public readonly expensePayment = input.required<IExpensesViewModel>();
	@Output() public displayExpenseDetail = new EventEmitter<number>();
}