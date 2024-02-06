import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormArray, FormControl, FormGroup, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute, Params } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { CheckboxModule } from 'primeng/checkbox';
import { DialogModule } from 'primeng/dialog';
import { InputNumberModule } from 'primeng/inputnumber';
import { InputTextModule } from 'primeng/inputtext';
import { MultiSelectModule } from 'primeng/multiselect';
import { firstValueFrom } from 'rxjs';
import { ExpenseType, IUser } from '../../../../../../modules';
import { AddExpenseModalApplication } from './add-expense-modal.application';

@Component({
	selector: 'app-add-expense-modal',
	templateUrl: './add-expense-modal.component.html',
	styleUrls: ['./add-expense-modal.component.scss'],
	standalone: true,
	imports: [DialogModule, FormsModule, ReactiveFormsModule, InputTextModule, MultiSelectModule, ButtonModule, InputNumberModule, CheckboxModule, CommonModule]
})
export class AddExpenseModalComponent implements OnInit {
	@Output() public expenseAdded = new EventEmitter<void>();
	@Input({ required: true }) public users: IUser[] = [];

	public form = {} as FormGroup<IAddExpenseForm>;
	public get payersControl(): FormArray<FormControl<number>> {
		return this.form.controls['payers'];
	}

	public get debtorsControl(): FormArray<FormControl<number>> {
		return this.form.controls['debtors'];
	}

	public readonly expenseTypeOptions: { name: string, id: ExpenseType } [] = [
		{ id: ExpenseType.Expense, name: 'Dépense' },
		{ id: ExpenseType.Gain, name: 'Rentrée d\'argent' },
		{ id: ExpenseType.Transfer, name: 'Transfert d\'argent' }
	]

	public display: boolean = false;
	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly addExpenseModalApplication: AddExpenseModalApplication
	) {}

	ngOnInit(): void {
		const payersAndDebtorsControls: FormControl[] = this.users.map(u => new FormControl())
		this.form = new FormGroup({
			name: new FormControl(),
			amount: new FormControl(),
			debtors: new FormArray([...payersAndDebtorsControls]),
			expenseType: new FormControl(),
			payers: new FormArray([...payersAndDebtorsControls]),
			description: new FormControl()
		});
	}

	public showDialog(): void {
		this.display = true;
	}

	public async onSubmitAsync(): Promise<void> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		const projectId = (params as { projectId: string })?.projectId

		await this.addExpenseModalApplication.addExpenseModalAsync(this.form, projectId);

		this.display = false;
		this.expenseAdded.next();
	}
}


export interface IAddExpenseForm {
	name: FormControl<string>;
	amount: FormControl<number>;
	debtors: FormArray<FormControl<number>>;
	expenseType: FormControl<ExpenseType[]>;
	payers: FormArray<FormControl<number>>;
	description: FormControl<Partial<string>>;
}