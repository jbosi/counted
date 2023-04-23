import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute, Params } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { InputNumberModule } from 'primeng/inputnumber';
import { MultiSelectModule } from 'primeng/multiselect';
import { firstValueFrom } from 'rxjs';
import { ExpenseType, ExpensesHttpClient, ICreatableExpense, IUser, IUserAmount } from '../../../../../../modules';

@Component({
	selector: 'app-add-expense-modal',
	templateUrl: './add-expense-modal.component.html',
	styleUrls: ['./add-expense-modal.component.scss'],
	standalone: true,
	imports: [DialogModule, FormsModule, ReactiveFormsModule, InputTextModule, MultiSelectModule, ButtonModule, InputNumberModule]
})
export class AddExpenseModalComponent implements OnInit {
	@Output() public expenseAdded = new EventEmitter<void>();
	@Input() public users: IUser[] = [];

	public form = {} as FormGroup<IAddExpenseForm>;

	public readonly expenseTypeOptions: { name: string, id: ExpenseType } [] = [
		{ id: ExpenseType.Expense, name: 'Dépense' },
		{ id: ExpenseType.Gain, name: 'Rentrée d\'argent' },
		{ id: ExpenseType.Transfer, name: 'Transfert d\'argent' }
	]

	public display: boolean = false;
	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly expensesHttpClient: ExpensesHttpClient
	) {}

	ngOnInit(): void {
		this.form = new FormGroup({
			name: new FormControl(),
			amount: new FormControl(),
			debtors: new FormControl(),
			expenseType: new FormControl(),
			payers: new FormControl(),
			description: new FormControl()
		})
	}

	public showDialog(): void {
		this.display = true;
	}

	public async onSubmitAsync(): Promise<void> {
		const amount: number = this.form.value?.amount as number;
		const payers: number[] = this.form?.value?.payers as number[];
		const debtors: number[] = this.form?.value?.debtors as number[];
		const candidate: ICreatableExpense = {
			name: this.form?.value?.name,
			amount: amount,
			expense_type: this.form?.value?.expenseType?.[0],
			debtors: debtors?.map(id => ({ amount: amount / (debtors.length), user_id: id })),
			payers: payers?.map(id => ({ amount: amount / (payers.length), user_id: id })),
			description: this.form?.value?.description,
			author_id: 1, // TODO
		} as ICreatableExpense

		const params: Params = await firstValueFrom(this.activatedRoute.params);
		const projectId = (params as { projectId: string })?.projectId

		await this.expensesHttpClient.createAsync(projectId, candidate);

		this.display = false;
		this.expenseAdded.next();
	}
}


interface IAddExpenseForm {
	name: FormControl<string>;
	amount: FormControl<number>;
	debtors: FormControl<number[]>;
	expenseType: FormControl<ExpenseType[]>;
	payers: FormControl<number[]>;
	description: FormControl<Partial<string>>;
}