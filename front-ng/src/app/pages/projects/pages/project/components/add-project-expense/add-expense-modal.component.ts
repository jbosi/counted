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
	@Input() public expenseId: number | undefined;

	public form = {} as FormGroup<IAddExpenseForm>;
	public get payersControl(): FormArray<FormGroup<IAddExpenseFormUserAmount>> {
		return this.form.controls['payers'];
	}

	public get debtorsControl(): FormArray<FormGroup<IAddExpenseFormUserAmount>> {
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
		const payersControls: FormGroup[] = this.users.map((_, index) => new FormGroup({
			isSelectedUser: new FormControl(index === 0),
			userAmount: new FormControl(0, { nonNullable: true })
		}));
		const debtorsControls: FormGroup[] = this.users.map(() => new FormGroup({
			isSelectedUser: new FormControl(true),
			userAmount: new FormControl(0, { nonNullable: true })
		}));

		this.form = new FormGroup({
			name: new FormControl('', { nonNullable: true }),
			amount: new FormControl<number>(0, { nonNullable: true }),
			debtors: new FormArray([...debtorsControls]),
			expenseType: new FormControl(),
			payers: new FormArray([...payersControls]),
			description: new FormControl()
		});

		if (this.expenseId != null) {
			this.setInitialValues(this.expenseId, this.form);
		}

		this.form.controls['amount'].valueChanges.subscribe(amount => {
			this.updateDebtorsAndPayors(this.form, amount);
		});

		[
			...this.form.controls['debtors'].controls,
			...this.form.controls['payers'].controls
		].forEach(c => c.valueChanges.subscribe(() => this.updateDebtorsAndPayors(this.form, this.form.controls['amount'].value)))
	}

	public showDialog(): void {
		this.display = true;
	}

	private setInitialValues(expenseId: number, form: FormGroup<IAddExpenseForm>): void {
		// this.addExpenseModalApplication.
	}

	public async onSubmitAsync(): Promise<void> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		const projectId = (params as { projectId: string })?.projectId

		await this.addExpenseModalApplication.addExpenseModalAsync(this.form, projectId);

		this.display = false;
		this.expenseAdded.next();
	}

	private updateDebtorsAndPayors(form: FormGroup<IAddExpenseForm>, amount: number): void {
		const dirtyPayersAmount: number = form.controls['payers'].controls
			.filter(c => c.controls['userAmount'].dirty && c.controls['isSelectedUser'].value)
			.reduce((acc, c) => acc + c.controls['userAmount'].value, 0);
		
		const pristinePayers = form.controls['payers'].controls
			.filter(c => c.controls['userAmount'].pristine && c.controls['isSelectedUser'].value);
		
		const totalPayers = pristinePayers.filter(c => c.controls['isSelectedUser'].value).length

		pristinePayers.forEach(c => c.controls['userAmount'].setValue((amount - dirtyPayersAmount) / totalPayers, { emitEvent: false }));

		const dirtyDebtorsAmount: number = form.controls['debtors'].controls
			.filter(c => c.controls['userAmount'].dirty && c.controls['isSelectedUser'].value)
			.reduce((acc, c) => acc + c.controls['userAmount'].value, 0);
		
		const pristineDebtors = form.controls['debtors'].controls
			.filter(c => c.controls['userAmount'].pristine && c.controls['isSelectedUser'].value);
		
		const totalDebtors = pristineDebtors.filter(c => c.controls['isSelectedUser'].value).length

		pristineDebtors.forEach(c => c.controls['userAmount'].setValue((amount - dirtyDebtorsAmount) / totalDebtors, { emitEvent: false }));
		
		// TODO Set errors if deb/pay amounts are not equal to expense amount
		if (true) {
			
		}

		// Reset amount for unSelectedUsers
		form.controls['debtors'].controls
			.filter(c => !c.get('isSelectedUser')?.value)
			.forEach(c => c.controls.userAmount.setValue(0, { emitEvent: false }));
		
		form.controls['payers'].controls
			.filter(c => !c.get('isSelectedUser')?.value)
			.forEach(c => c.controls.userAmount.setValue(0, { emitEvent: false }));
	}
}


export interface IAddExpenseForm {
	name: FormControl<string>;
	amount: FormControl<number>;
	debtors: FormArray<FormGroup<IAddExpenseFormUserAmount>>;
	expenseType: FormControl<ExpenseType[]>;
	payers: FormArray<FormGroup<IAddExpenseFormUserAmount>>;
	description: FormControl<Partial<string>>;
}

export interface IAddExpenseFormUserAmount {
	isSelectedUser: FormControl<boolean>
	userAmount: FormControl<number>
}