import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useMemo, useState, type RefObject } from 'react';
import { useFieldArray, useForm, useWatch, type FieldArrayWithId, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import * as z from 'zod';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { useAddExpense } from '../../../hooks/useExpenses';
import { type CreatableExpense, type ExpenseType } from '../../../types/expenses.model';
import type { User } from '../../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import { expenseFormSchema, getDebtorsFieldLabel, getPayersFieldLabel } from './helpers/expenseModal.helper';

export interface AddExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
	closeDialogFn: () => void;
}

export type AddExpenseModalForm = z.infer<typeof expenseFormSchema>;
type FormCheckbox = AddExpenseModalForm['payers'][number];

interface FormCheckboxProps {
	user: User;
	register: UseFormRegister<AddExpenseModalForm>;
	getValues: UseFormGetValues<AddExpenseModalForm>;
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>;
	fields: FieldArrayWithId<AddExpenseModalForm>[];
	type: 'debtors' | 'payers';
	amount: number;
	isChecked: boolean;
	index: number;
	shareMode?: boolean;
	shares?: number;
	onSharesChange?: (userId: number, shares: number) => void;
}

function getInitialValues(users: User[]): Partial<AddExpenseModalForm> {
	const initialDebtorsFormCheckBoxValues: FormCheckbox[] = users.map((u) => ({
		amount: 0,
		isChecked: true,
		user: u,
	}));

	const initialPayersFormCheckBoxValues: FormCheckbox[] = users.map((u) => ({
		amount: 0,
		isChecked: false,
		user: u,
	}));

	return {
		payers: initialPayersFormCheckBoxValues,
		debtors: initialDebtorsFormCheckBoxValues,
		totalAmount: 0,
		name: '',
		date: new Date().toLocaleDateString('en-CA'),
	};
}

export function AddExpenseModal({ dialogRef, modalId, users, projectId, closeDialogFn }: AddExpenseModalProps) {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const defaultValues = useMemo(() => getInitialValues(users), [users]);

	const {
		register,
		handleSubmit,
		formState: { errors },
		getValues,
		control,
	} = useForm<AddExpenseModalForm>({
		defaultValues: defaultValues,
		resolver: zodResolver(expenseFormSchema),
	});

	const expenseType = useWatch({
		control,
		name: 'type',
		defaultValue: defaultValues.type,
	});

	const { fields: payersFields, update: updatePayer } = useFieldArray({ control, name: 'payers' });
	const { fields: debtorsfields, update: updateDebtor } = useFieldArray({ control, name: 'debtors' });

	const [payersShareMode, setPayersShareMode] = useState(false);
	const [debtorsShareMode, setDebtorsShareMode] = useState(false);
	const [payersShares, setPayersShares] = useState<Record<number, number>>(() => Object.fromEntries(users.map((u) => [u.id, 0])));
	const [debtorsShares, setDebtorsShares] = useState<Record<number, number>>(() => Object.fromEntries(users.map((u) => [u.id, 1])));

	const handlePayerSharesChange = useCallback(
		(userId: number, newShares: number) => {
			const next = { ...payersShares, [userId]: newShares };
			setPayersShares(next);
			updateAmounts('payers', getValues(), updatePayer, payersFields, next);
		},
		[payersShares, getValues, updatePayer, payersFields],
	);

	const handleDebtorSharesChange = useCallback(
		(userId: number, newShares: number) => {
			const next = { ...debtorsShares, [userId]: newShares };
			setDebtorsShares(next);
			updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, next);
		},
		[debtorsShares, getValues, updateDebtor, debtorsfields],
	);

	const togglePayersShareMode = useCallback(() => {
		const newMode = !payersShareMode;
		setPayersShareMode(newMode);
		updateAmounts('payers', getValues(), updatePayer, payersFields, newMode ? payersShares : undefined);
	}, [payersShareMode, payersShares, getValues, updatePayer, payersFields]);

	const toggleDebtorsShareMode = useCallback(() => {
		const newMode = !debtorsShareMode;
		setDebtorsShareMode(newMode);
		updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, newMode ? debtorsShares : undefined);
	}, [debtorsShareMode, debtorsShares, getValues, updateDebtor, debtorsfields]);

	const { mutate, isPending, isError, error } = useAddExpense();

	const onSubmit = useCallback(
		(formValues: AddExpenseModalForm) => {
			const creatableExpense: CreatableExpense = {
				name: formValues.name,
				amount: formValues.totalAmount,
				expenseType: formValues.type,
				projectId,
				payers: formValues.payers.map((p) => ({ amount: p.amount, userId: p.user.id })),
				debtors: formValues.debtors.map((p) => ({ amount: p.amount, userId: p.user.id })),
				authorId: getProjectUserIdFromLocalstorage(countedLocalStorage, projectId) ?? users[0].id, // TODO
				date: formValues.date,
			};

			mutate(creatableExpense);
			closeDialogFn();
		},
		[closeDialogFn, mutate, projectId, users, countedLocalStorage],
	);

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={closeDialogFn}>
						✕
					</button>
					<h1>Ajouter une dépense</h1>
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
										updateAmounts('debtors', getValues(), updateDebtor, debtorsfields, debtorsShareMode ? debtorsShares : undefined);
										updateAmounts('payers', getValues(), updatePayer, payersFields, payersShareMode ? payersShares : undefined);
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
										shareMode={payersShareMode}
										shares={payersShares[field.user.id] ?? 1}
										onSharesChange={handlePayerSharesChange}
									/>
								))}
							</fieldset>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">{getDebtorsFieldLabel(expenseType)}</legend>
								<label className="label justify-end gap-2 mb-1">
									<span className="text-xs">Par parts</span>
									<input type="checkbox" className="toggle toggle-sm" checked={debtorsShareMode} onChange={toggleDebtorsShareMode} />
								</label>
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
										shareMode={debtorsShareMode}
										shares={debtorsShares[field.user.id] ?? 1}
										onSharesChange={handleDebtorSharesChange}
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
	fields: FieldArrayWithId<AddExpenseModalForm>[];
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm, T>;
	getValues: UseFormGetValues<AddExpenseModalForm>;
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
	values: AddExpenseModalForm,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm, 'debtors' | 'payers'>,
	debtorsOrPayersfields: FieldArrayWithId<AddExpenseModalForm>[],
	sharesMap?: Record<number, number>,
) {
	const debtorsOrPayers = values[type];
	const totalAmountValue = values.totalAmount;

	const activeDebtorOrPayersFields = debtorsOrPayers.filter((field) => field.isChecked);
	const activeDebtorOrPayersCount = activeDebtorOrPayersFields.length;

	if (activeDebtorOrPayersCount === 0) {
		return;
	}

	if (sharesMap) {
		const totalShares = activeDebtorOrPayersFields.reduce((sum, f) => sum + (sharesMap[f.user.id] ?? 1), 0);
		if (totalShares === 0) return;
		activeDebtorOrPayersFields.forEach((field) => {
			const shares = sharesMap[field.user.id] ?? 1;
			const amount = parseFloat(((shares / totalShares) * totalAmountValue).toFixed(2));
			updateMethod(
				debtorsOrPayersfields.findIndex((f) => f.user.id === field.user.id),
				{ amount, isChecked: field.isChecked, user: field.user },
			);
		});
	} else {
		const updatedAndRoundedDebtorOrPayersAmount = parseFloat((totalAmountValue / activeDebtorOrPayersCount).toFixed(2));
		activeDebtorOrPayersFields.forEach((field) => {
			updateMethod(
				debtorsOrPayersfields.findIndex((f) => f.user.id === field.user.id),
				{ amount: updatedAndRoundedDebtorOrPayersAmount, isChecked: field.isChecked, user: field.user },
			);
		});
	}
}

export function FormCheckbox({ isChecked, register, type, user, index, getValues, updateMethod, fields, shareMode, shares, onSharesChange }: FormCheckboxProps) {
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
							resetExpenseAmountOnUnchecked(getValues, type, index, updateMethod, user);
							if (shareMode) {
								const newShare = isNowChecked ? Math.max(1, shares ?? 1) : 0;
								onSharesChange?.(user.id, newShare);
							} else {
								updateAmounts(type, getValues(), updateMethod, fields, undefined);
							}
						},
					})}
				/>
				{user.name}
			</div>
			{shareMode ? (
				<div className="flex items-center gap-1.5">
					<input
						className="input w-20"
						type="number"
						min="1"
						step="1"
						value={shares ?? 1}
						onChange={(e) => onSharesChange?.(user.id, Math.max(1, parseInt(e.target.value) || 1))}
					/>
					<span className="text-xs opacity-60">part(s)</span>
				</div>
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

function resetExpenseAmountOnUnchecked(
	getValues: UseFormGetValues<AddExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>,
	user: User,
) {
	const isChecked = getValues(`${type}.${index}.isChecked`);
	if (!isChecked) {
		updateMethod(index, { amount: 0, isChecked, user });
	}
}

function toggleCheckedIfAmountChange(
	getValues: UseFormGetValues<AddExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>,
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
