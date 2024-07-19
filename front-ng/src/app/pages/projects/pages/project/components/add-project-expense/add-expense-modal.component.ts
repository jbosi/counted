
import { Component, OnInit } from '@angular/core';
import { FormArray, FormControl, FormGroup, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ActivatedRoute, Params } from '@angular/router';
import { ButtonModule } from 'primeng/button';
import { CheckboxModule } from 'primeng/checkbox';
import { DialogModule } from 'primeng/dialog';
import { DynamicDialogConfig, DynamicDialogRef } from 'primeng/dynamicdialog';
import { InputNumberModule } from 'primeng/inputnumber';
import { InputTextModule } from 'primeng/inputtext';
import { MultiSelectModule } from 'primeng/multiselect';
import { firstValueFrom } from 'rxjs';
import { AvatarInitialsComponent, ExpenseType, IExpensesViewModel, IUser } from '../../../../../../modules';
import { IExpenseModalData } from '../../project.component';
import { AddExpenseModalApplication } from './add-expense-modal.application';

@Component({
	selector: 'app-add-expense-modal',
	templateUrl: './add-expense-modal.component.html',
	styleUrls: ['./add-expense-modal.component.scss'],
	standalone: true,
	imports: [DialogModule, FormsModule, ReactiveFormsModule, InputTextModule, MultiSelectModule, ButtonModule, InputNumberModule, CheckboxModule, AvatarInitialsComponent]
})
export class AddExpenseModalComponent implements OnInit {
	public users: IUser[] = [];
	public expense: IExpensesViewModel | undefined;

	public form = {} as FormGroup<IAddExpenseForm>;
	public get payersControl(): FormArray<FormGroup<IAddExpenseFormUserAmount>> {
		return this.form.controls['payers'];
	}

	public get debtorsControl(): FormArray<FormGroup<IAddExpenseFormUserAmount>> {
		return this.form.controls['debtors'];
	}

	public readonly expenseTypeOptions: { name: string, id: ExpenseType }[] = [
		{ id: ExpenseType.Expense, name: 'Dépense' },
		{ id: ExpenseType.Gain, name: 'Rentrée d\'argent' },
		{ id: ExpenseType.Transfer, name: 'Transfert d\'argent' }
	]

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly addExpenseModalApplication: AddExpenseModalApplication,
		private readonly modal: DynamicDialogRef,
		private readonly modalConfig: DynamicDialogConfig
	) {}

	ngOnInit(): void {
		const config: DynamicDialogConfig<IExpenseModalData> = this.modalConfig

		this.users = config.data?.users ?? [];
		this.expense = config.data?.expensePayment;

		this.form = this.getFormGroup(this.expense)

		this.form.controls['amount'].valueChanges.subscribe(amount => {
			this.updateDebtorsAndPayors(this.form, amount);
		});

		[
			...this.form.controls['debtors'].controls,
			...this.form.controls['payers'].controls
		].forEach(c => c.valueChanges.subscribe(() => this.updateDebtorsAndPayors(this.form, this.form.controls['amount'].value)))
	}

	private getFormGroup(expense: IExpensesViewModel | undefined): FormGroup<IAddExpenseForm> {
		const hasNoSelectedPayors: boolean = expense?.payors.length === 0;
		const payersControls: FormGroup[] = this.users.map((u, index) => new FormGroup({
			isSelectedUser: new FormControl(hasNoSelectedPayors ? index === 0 : expense?.payors?.[index] != null),
			userAmount: new FormControl(expense?.payors?.[index]?.amount ?? 0, { nonNullable: true }),
			userId: new FormControl(u.id ?? 0, { nonNullable: true }) // usefull for POST only
		}));

		const hasNoSelectedDebtors: boolean = expense?.debtors.length === 0;
		const debtorsControls: FormGroup[] = this.users.map((u, index) => new FormGroup({
			isSelectedUser: new FormControl(hasNoSelectedDebtors ? true : expense?.debtors?.[index] != null),
			userAmount: new FormControl(expense?.debtors?.[index]?.amount ?? 0, { nonNullable: true }),
			userId: new FormControl(u.id, { nonNullable: true })
		}));

		const form: FormGroup<IAddExpenseForm> = new FormGroup({
			name: new FormControl<string>(expense?.name ?? '', { nonNullable: true }),
			amount: new FormControl(expense?.amount ?? 0, { nonNullable: true }),
			debtors: new FormArray([...debtorsControls]),
			expenseType: new FormControl(this.getExpenseTypeInitialValue(expense) as any as ({ name: string; id: ExpenseType; })[], { nonNullable: true }),
			payers: new FormArray([...payersControls]),
			description: new FormControl(expense?.description ?? '')
		});

		return form;
	}

	public async onSubmitAsync(): Promise<void> {
		const params: Params = await firstValueFrom(this.activatedRoute.params);
		const projectId = (params as { projectId: string })?.projectId

		await this.addExpenseModalApplication.createOrEditExpenseAsync(this.form, projectId, this.expense?.id);

		this.modal.close(true);
	}

	private getExpenseTypeInitialValue(expense: IExpensesViewModel | undefined): ExpenseType[] {
		if (!expense?.expense_type) {
			return [];
		}
		
		const value = this.expenseTypeOptions.find(eto => eto.id === expense?.expense_type) ?? this.expenseTypeOptions[0];
		return [value.id];
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
	expenseType: FormControl<{ name: string, id: ExpenseType }[]>;
	payers: FormArray<FormGroup<IAddExpenseFormUserAmount>>;
	description: FormControl<string | null>;
}

export interface IAddExpenseFormUserAmount {
	isSelectedUser: FormControl<boolean>
	userAmount: FormControl<number>
	userId: FormControl<number>
}