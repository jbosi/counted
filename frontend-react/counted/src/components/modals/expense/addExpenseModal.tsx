import { useCallback, useContext, useMemo, type RefObject } from 'react';
import { useFieldArray, useForm, useWatch, type FieldArrayWithId, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';
import { useAddExpense } from '../../../hooks/useExpenses';
import { ExpenseTypeConst, type CreatableExpense, type ExpenseType } from '../../../types/expenses.model';
import type { User } from '../../../types/users.model';
import { ErrorValidationCallout } from '../../errorCallout';
import { getDebtorsFieldLabel, getPayersFieldLabel } from './helpers/expenseModal.helper';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';

export interface AddExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
	closeDialogFn: () => void;
}

const userSchema = z.object({
	id: z.number(),
	name: z.string(),
	balance: z.number().nullish(),
	created_at: z.string().nullish(),
});

const CheckboxFormSchema = z.object({
	amount: z.number().min(0),
	isChecked: z.boolean(),
	user: userSchema,
});

const formSchema = z.object({
	name: z.string().min(2).max(100),
	description: z.string().max(200).optional(),
	totalAmount: z.number().min(0.01).max(100000),
	type: z.enum(ExpenseTypeConst),
	date: z.string().min(1, 'La date est requise'),
	payers: z.array(CheckboxFormSchema).min(1),
	debtors: z.array(CheckboxFormSchema).min(1),
});

export type AddExpenseModalForm = z.infer<typeof formSchema>;
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
		description: '',
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
		resolver: zodResolver(formSchema),
	});

	const expenseType = useWatch({
		control,
		name: 'type',
		defaultValue: defaultValues.type,
	});

	const { fields: payersFields, update: updatePayer } = useFieldArray({ control, name: 'payers' });
	const { fields: debtorsfields, update: updateDebtor } = useFieldArray({ control, name: 'debtors' });

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
