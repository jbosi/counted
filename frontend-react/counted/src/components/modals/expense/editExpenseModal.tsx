import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useMemo, useState, type RefObject } from 'react';
import { useFieldArray, useForm, useWatch, type FieldArrayWithId, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import * as z from 'zod';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { useEditExpense } from '../../../hooks/useExpenses';
import { type EditableExpense, type Expense, type ExpenseType } from '../../../types/expenses.model';
import type { Payment } from '../../../types/payments.model';
import type { User } from '../../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import {
	expenseFormSchema,
	getDebtorsFieldLabel,
	getPayersFieldLabel,
	resetExpenseAmountOnUnchecked,
	toggleCheckedIfAmountChange,
	updateAmounts,
} from './helpers/expenseModal.helper';
import { SelectAllCheckboxInput } from './components/selectAllCheckboxInput';
import { ModalFooter } from '../shared/modalFooter';
import { ExpenseShareInput } from './components/expenseShareInput';
import { getPickerFormattedDate } from '../../../utils/date';

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
	shareMode?: boolean;
	onRecalculate?: () => void;
}

function getInitialValues(users: User[], expense: Expense, payments: Payment[]): Partial<EditExpenseModalForm> {
	const initialDebtorsFormCheckBoxValues: FormCheckbox[] = users.map((u) => {
		const payment = payments.find((p) => p.isDebt && p.userId === u.id);
		return {
			amount: payment?.amount ?? 0,
			shares: payment != null ? 1 : 0,
			isChecked: payment != null,
			user: u,
		};
	});

	const initialPayersFormCheckBoxValues: FormCheckbox[] = users.map((u) => {
		const payment = payments.find((p) => !p.isDebt && p.userId === u.id);
		return {
			amount: payment?.amount ?? 0,
			shares: payment != null ? 1 : 0,
			isChecked: payment != null,
			user: u,
		};
	});

	return {
		payers: initialPayersFormCheckBoxValues,
		debtors: initialDebtorsFormCheckBoxValues,
		totalAmount: expense.amount ?? 0,
		name: expense.name ?? '',
		type: expense.expenseType,
		date: expense.date ?? getPickerFormattedDate(new Date()),
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

	const [payersShareMode, setPayersShareMode] = useState(false);
	const [debtorsShareMode, setDebtorsShareMode] = useState(false);

	const handlePayersRecalculate = useCallback(() => {
		updateAmounts('payers', getValues(), updatePayer, payersFields, payersShareMode);
	}, [getValues, updatePayer, payersFields, payersShareMode]);

	const handleDebtorsRecalculate = useCallback(() => {
		updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, debtorsShareMode);
	}, [getValues, updateDebtor, debtorsfields, debtorsShareMode]);

	const togglePayersShareMode = useCallback(() => {
		const newMode = !payersShareMode;
		setPayersShareMode(newMode);
		updateAmounts('payers', getValues(), updatePayer, payersFields, newMode);
	}, [payersShareMode, getValues, updatePayer, payersFields]);

	const toggleDebtorsShareMode = useCallback(() => {
		const newMode = !debtorsShareMode;
		setDebtorsShareMode(newMode);
		updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, newMode);
	}, [debtorsShareMode, getValues, updateDebtor, debtorsfields]);

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
										updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, debtorsShareMode);
										updateAmounts('payers', getValues(), updatePayer, payersFields, payersShareMode);
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
								<label className="label justify-end gap-2 mb-1">
									<span className="text-xs">Par parts</span>
									<input type="checkbox" className="toggle toggle-sm" checked={payersShareMode} onChange={togglePayersShareMode} />
								</label>
								<SelectAllCheckboxInput initialValue={false} fields={payersFields} updateMethod={updatePayer} getValues={getValues} type={'payers'} />
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
										shareMode={payersShareMode}
										onRecalculate={handlePayersRecalculate}
									/>
								))}
							</fieldset>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">{getDebtorsFieldLabel(expenseType)}</legend>
								<label className="label justify-end gap-2 mb-1">
									<span className="text-xs">Par parts</span>
									<input type="checkbox" className="toggle toggle-sm" checked={debtorsShareMode} onChange={toggleDebtorsShareMode} />
								</label>
								<SelectAllCheckboxInput initialValue={true} fields={debtorsfields} updateMethod={updateDebtor} getValues={getValues} type={'debtors'} />
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
										shareMode={debtorsShareMode}
										onRecalculate={handleDebtorsRecalculate}
									/>
								))}
							</fieldset>

							{isPending && <span className="loading loading-spinner loading-xs"></span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

						<ModalFooter closeDialogFn={closeDialogFn} />
					</form>
				</div>
			</dialog>
		</>
	);
}

export function FormCheckbox({ isChecked, register, type, user, index, getValues, updateMethod, fields, amount, shareMode, onRecalculate }: FormCheckboxProps) {
	return (
		<label className="label justify-between">
			<div className="flex gap-2">
				<input
					type="checkbox"
					defaultChecked={isChecked ?? undefined}
					className="checkbox"
					{...register(`${type}.${index}.isChecked`, {
						onChange() {
							const isNowChecked = getValues(`${type}.${index}.isChecked`);
							if (shareMode) {
								const currentShares = getValues(`${type}.${index}.shares`) ?? 0;
								const newShares = isNowChecked ? Math.max(1, currentShares) : 0;
								updateMethod(index, { amount: 0, isChecked: isNowChecked, user, shares: newShares });
								onRecalculate?.();
							} else {
								resetExpenseAmountOnUnchecked(getValues, type, index, updateMethod, user);
								updateAmounts(type, getValues(), updateMethod, fields, false);
							}
						},
					})}
				/>
				{user.name}
			</div>
			{shareMode ? (
				<ExpenseShareInput
					amount={amount}
					user={user}
					index={index}
					register={register}
					getValues={getValues}
					updateMethod={updateMethod}
					type={type}
					onRecalculate={onRecalculate}
				/>
			) : (
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
			)}
		</label>
	);
}
