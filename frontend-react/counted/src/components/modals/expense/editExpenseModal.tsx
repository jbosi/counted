import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useMemo, type RefObject } from 'react';
import { useFieldArray, useForm, useWatch, type FieldArrayWithId, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import * as z from 'zod';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { useEditExpense } from '../../../hooks/useExpenses';
import { type EditableExpense, type Expense, type ExpenseType } from '../../../types/expenses.model';
import type { Payment } from '../../../types/payments.model';
import type { User } from '../../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import { expenseFormSchema, getDebtorsFieldLabel, getPayersFieldLabel } from './helpers/expenseModal.helper';

export interface EditExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
	expense: Expense;
	payments: Payment[];
	closeDialogFn: () => void;
}

export type EditExpenseModalForm = z.infer<typeof expenseFormSchema>;
type FormCheckbox = EditExpenseModalForm['payers'][number];

interface FormCheckboxProps {
	user: User;
	register: UseFormRegister<EditExpenseModalForm>;
	getValues: UseFormGetValues<EditExpenseModalForm>;
	updateMethod: UseFieldArrayUpdate<EditExpenseModalForm>;
	fields: FieldArrayWithId<EditExpenseModalForm>[];
	type: 'debtors' | 'payers';
	amount: number;
	isChecked: boolean;
	index: number;
}

function getInitialValues(users: User[], expense: Expense, payments: Payment[]): Partial<EditExpenseModalForm> {
	const initialDebtorsFormCheckBoxValues: FormCheckbox[] = users.map((u) => {
		const payment = payments.find((p) => p.isDebt && p.userId === u.id);
		return {
			amount: payment?.amount ?? 0,
			isChecked: payment != null,
			user: u,
		};
	});

	const initialPayersFormCheckBoxValues: FormCheckbox[] = users.map((u) => {
		const payment = payments.find((p) => !p.isDebt && p.userId === u.id);
		return {
			amount: payment?.amount ?? 0,
			isChecked: payment != null,
			user: u,
		};
	});

	return {
		payers: initialPayersFormCheckBoxValues,
		debtors: initialDebtorsFormCheckBoxValues,
		totalAmount: expense.amount,
		name: expense.name,
		description: expense.description ?? '',
		type: expense.expenseType,
		date: expense.date,
	};
}

export function EditExpenseModal({ dialogRef, modalId, users, projectId, expense, payments, closeDialogFn }: EditExpenseModalProps) {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const defaultValues = useMemo(() => getInitialValues(users, expense, payments), [users, expense, payments]);
	const {
		register,
		handleSubmit,
		formState: { errors },
		getValues,
		control,
	} = useForm<EditExpenseModalForm>({
		defaultValues: defaultValues,
		resolver: zodResolver(expenseFormSchema),
	});

	const { fields: payersFields, update: updatePayer } = useFieldArray({ control, name: 'payers' });
	const { fields: debtorsfields, update: updateDebtor } = useFieldArray({ control, name: 'debtors' });

	const { mutate, isPending, isError, error } = useEditExpense();

	const expenseType = useWatch({
		control,
		name: 'type',
		defaultValue: defaultValues.type,
	});

	const onSubmit = useCallback(
		(formValues: EditExpenseModalForm) => {
			const editableExpense: EditableExpense = {
				id: expense.id,
				name: formValues.name,
				amount: formValues.totalAmount,
				expenseType: formValues.type,
				projectId,
				payers: formValues.payers.map((p) => ({ amount: p.amount, userId: p.user.id })),
				debtors: formValues.debtors.map((p) => ({ amount: p.amount, userId: p.user.id })),
				authorId: getProjectUserIdFromLocalstorage(countedLocalStorage, projectId) ?? users[0].id,
				date: formValues.date,
			};

			mutate(editableExpense);
			closeDialogFn();
		},
		[countedLocalStorage, closeDialogFn, mutate, projectId, users, expense],
	);

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={closeDialogFn}>
						✕
					</button>
					<h1>Editer une dépense</h1>
					<ErrorValidationCallout errors={errors} />
					<form className="ml-4 mr-4" onSubmit={handleSubmit(onSubmit)}>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('name')} />
							{errors.name && <span>Ce champ est requis</span>}

							<label className="label">Description</label>
							<input className="input w-full" {...register('description')} />

							<label className="label">Date</label>
							<input className="input w-full" type="date" {...register('date')} />

							<label className="label">Montant</label>
							<input
								min="0"
								className="input w-full"
								step="0.01"
								type="number"
								{...register('totalAmount', {
									valueAsNumber: true,
									onBlur() {
										updateAmounts('debtors', getValues(), updateDebtor, debtorsfields);
										updateAmounts('payers', getValues(), updatePayer, payersFields);
									},
								})}
							/>

							<label className="label">Type de dépense</label>
							<select defaultValue="Dépense" className="select w-full" {...register('type')}>
								<option value={'Expense' as ExpenseType}>Dépense</option>
								<option value={'Gain' as ExpenseType}>Gain</option>
								<option value={'Transfer' as ExpenseType}>Transfert d'argent</option>
							</select>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">{getPayersFieldLabel(expenseType)}</legend>
								<SelectAllCheckbox initialValue={false} fields={payersFields} updateMethod={updatePayer} getValues={getValues} type={'payers'} />
								{payersFields.map((field, index) => (
									<FormCheckbox
										key={field.id}
										amount={field.amount}
										isChecked={field.isChecked}
										user={field.user}
										index={index}
										register={register}
										getValues={getValues}
										updateMethod={updatePayer}
										fields={payersFields}
										type="payers"
									/>
								))}
							</fieldset>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">{getDebtorsFieldLabel(expenseType)}</legend>
								<SelectAllCheckbox initialValue={true} fields={debtorsfields} updateMethod={updateDebtor} getValues={getValues} type={'debtors'} />
								{debtorsfields.map((field, index) => (
									<FormCheckbox
										key={field.id}
										amount={field.amount}
										isChecked={field.isChecked}
										user={field.user}
										index={index}
										register={register}
										getValues={getValues}
										updateMethod={updateDebtor}
										fields={debtorsfields}
										type="debtors"
									/>
								))}
							</fieldset>

							{isPending && <span className="loading loading-spinner loading-xs"></span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className="btn btn-primary" type="submit">
								Enregistrer
							</button>
							<button className="btn btn-outline" type="button" onClick={closeDialogFn}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}

interface SelectAllCheckboxProps<T extends 'debtors' | 'payers' = 'debtors' | 'payers'> {
	type: T;
	fields: FieldArrayWithId<EditExpenseModalForm>[];
	updateMethod: UseFieldArrayUpdate<EditExpenseModalForm, T>;
	getValues: UseFormGetValues<EditExpenseModalForm>;
	initialValue: boolean;
}

function SelectAllCheckbox({ type, fields, updateMethod, getValues, initialValue }: SelectAllCheckboxProps) {
	return (
		<label className="label justify-between mb-2">
			<div className="flex gap-2">
				<input
					type="checkbox"
					defaultChecked={initialValue}
					className="checkbox"
					onClick={(e) => {
						const isChecked = (e.target as HTMLInputElement).checked;
						fields.forEach((p, index) => {
							updateMethod(index, { ...p, isChecked });
							resetExpenseAmountOnUnchecked(getValues, type, index, updateMethod, p.user);
						});
						updateAmounts(type, getValues(), updateMethod, fields);
					}}
				/>
				Selectionner tous les utilisateurs
			</div>
		</label>
	);
}

function updateAmounts<T extends 'debtors' | 'payers'>(
	type: T,
	values: EditExpenseModalForm,
	updateMethod: UseFieldArrayUpdate<EditExpenseModalForm, 'debtors' | 'payers'>,
	debtorsOrPayersfields: FieldArrayWithId<EditExpenseModalForm>[],
) {
	const debtorsOrPayers = values[type];
	const totalAmountValue = values.totalAmount;

	const activeDebtorOrPayersFields = debtorsOrPayers.filter((field) => field.isChecked);
	const activeDebtorOrPayersCount = activeDebtorOrPayersFields.length;

	if (activeDebtorOrPayersCount === 0) {
		return;
	}

	const updatedAndRoundedDebtorOrPayersAmount = parseFloat((totalAmountValue / activeDebtorOrPayersCount).toFixed(2));

	activeDebtorOrPayersFields.forEach((field) => {
		updateMethod(
			debtorsOrPayersfields.findIndex((f) => f.user.id === field.user.id),
			{ amount: updatedAndRoundedDebtorOrPayersAmount, isChecked: field.isChecked, user: field.user },
		);
	});
}

export function FormCheckbox({ isChecked, register, type, user, index, getValues, updateMethod, fields }: FormCheckboxProps) {
	return (
		<label className="label justify-between">
			<div className="flex gap-2">
				<input
					type="checkbox"
					defaultChecked={isChecked ?? undefined}
					className="checkbox"
					{...register(`${type}.${index}.isChecked`, {
						onChange() {
							resetExpenseAmountOnUnchecked(getValues, type, index, updateMethod, user);
							updateAmounts(type, getValues(), updateMethod, fields);
						},
					})}
				/>
				{user.name}
			</div>
			<input
				className="input w-44"
				type="number"
				step="0.01"
				{...register(`${type}.${index}.amount`, {
					valueAsNumber: true,
					onBlur() {
						toggleCheckedIfAmountChange(getValues, type, index, updateMethod, user);
					},
				})}
			/>
		</label>
	);
}

function resetExpenseAmountOnUnchecked(
	getValues: UseFormGetValues<EditExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<EditExpenseModalForm>,
	user: User,
) {
	const isChecked = getValues(`${type}.${index}.isChecked`);
	if (!isChecked) {
		updateMethod(index, { amount: 0, isChecked, user });
	}
}

function toggleCheckedIfAmountChange(
	getValues: UseFormGetValues<EditExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<EditExpenseModalForm>,
	user: User,
) {
	const amount = getValues(`${type}.${index}.amount`);
	const isChecked = getValues(`${type}.${index}.isChecked`);
	if (amount > 0 && !isChecked) {
		updateMethod(index, { amount, isChecked: true, user });
	} else if (amount === 0 && isChecked) {
		updateMethod(index, { amount, isChecked: false, user });
	}
}
