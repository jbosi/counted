import { NgIf } from '@angular/common';
import { ChangeDetectionStrategy, Component, Input } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { IExpense } from 'src/app/modules';

@Component({
	selector: 'app-expense',
	templateUrl: './expense.component.html',
	styleUrls: ['./expense.component.scss'],
	standalone: true,
	imports: [AvatarModule, NgIf],
	changeDetection: ChangeDetectionStrategy.OnPush
})
export class ExpenseComponent {
	@Input() public expense!: IExpense;

	constructor() {}
}