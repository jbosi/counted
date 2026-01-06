import { useCallback, useMemo, useState, type FormEvent, type RefObject } from 'react';
import { useFieldArray, useForm, type FieldArrayWithId, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import * as z from 'zod';
import { useAddExpense } from '../../../hooks/useExpenses';
import { ExpenseTypeConst, type CreatableExpense, type ExpenseType } from '../../../types/expenses.model';
import type { User } from '../../../types/users.model';
import { ErrorValidationCallout } from '../../errorCallout';

export interface AddExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
}

export interface AddExpenseModalForm {
	name: string;
	description: string;
	totalAmount: number;
	type: ExpenseType;
	payers: FormCheckbox[];
	debtors: FormCheckbox[];
}

const payersAndDebtorsForm = z.object({
	amount: z.number().min(0),
	isChecked: z.boolean(),
	user: z.object().optional(),
});

const formSchema = z.object({
	name: z.string().min(2).max(100),
	description: z.string().max(200).optional(),
	totalAmount: z.number().min(0.01).max(100000),
	type: z.literal(ExpenseTypeConst),
	payers: z.array(payersAndDebtorsForm).min(1),
	debtors: z.array(payersAndDebtorsForm).min(1),
});

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

interface FormCheckbox {
	user: User;
	isChecked: boolean;
	amount: number;
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
	};
}

export function AddExpenseModal({ dialogRef, modalId, users, projectId }: AddExpenseModalProps) {
	const [errorState, setErrorState] = useState<string | null>(null);
	const defaultValues = useMemo(() => getInitialValues(users), [users]);

	const {
		register,
		formState: { errors, isDirty },
		getValues,
		control,
		reset,
	} = useForm<AddExpenseModalForm>({
		defaultValues: defaultValues,
	});

	const exitModal = useCallback(() => {
		reset(defaultValues);
		dialogRef.current?.close();
	}, [reset, defaultValues, dialogRef]);

	const { fields: payersFields, update: updatePayer } = useFieldArray({ control, name: 'payers' });
	const { fields: debtorsfields, update: updateDebtor } = useFieldArray({ control, name: 'debtors' });

	const { mutate, isPending, isError, error } = useAddExpense();

	const handleSubmit = useCallback(
		(e: FormEvent<HTMLFormElement>) => {
			e.preventDefault();

			const formValues = getValues();
			const parsedResult = formSchema.safeParse(formValues);

			if (parsedResult.error) {
				setErrorState(parsedResult.error.message);
				return;
			}

			const creatableExpense: CreatableExpense = {
				name: formValues.name,
				amount: formValues.totalAmount,
				expenseType: formValues.type,
				projectId,
				payers: formValues.payers.map((p) => ({ amount: p.amount, userId: p.user.id })),
				debtors: formValues.debtors.map((p) => ({ amount: p.amount, userId: p.user.id })),
				authorId: 41, // TODO
			};

			mutate(creatableExpense);
			exitModal();
		},
		[exitModal, getValues, mutate, projectId],
	);

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={exitModal}>
						✕
					</button>
					<h1>Ajouter une dépense</h1>
					<ErrorValidationCallout errorState={errorState} /> {/* TODO, use error boundary ? */}
					<form className="ml-4 mr-4" onSubmit={handleSubmit}>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('name', { required: true, maxLength: 100 })} />
							{errors.name && <span>Ce champ est requis</span>}

							<label className="label">Description</label>
							<input className="input w-full" {...register('description', { maxLength: 200 })} />

							<label className="label">Valeur</label>
							<input
								min="0"
								className="input w-full"
								step="0.01"
								type="number"
								{...register('totalAmount', {
									required: true,
									valueAsNumber: true,
									onBlur() {
										updateAmounts('debtors', getValues, updateDebtor, debtorsfields);
										updateAmounts('payers', getValues, updatePayer, payersFields);
									},
								})}
							/>

							<label className="label">Type de dépense</label>
							<select defaultValue="Dépense" className="select w-full" {...register('type', { required: true })}>
								<option value={'Expense'}>Dépense</option>
								<option value={'Gain'}>Gain</option>
								<option value={'Transfert'}>Transfert d'argent</option>
							</select>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">Qui a payé ?</legend>
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
								<legend className="fieldset-legend">Qui doit rembourser ?</legend>
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
							<button className="btn btn-outline" type="button" onClick={exitModal}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}

function updateAmounts<T extends 'debtors' | 'payers'>(
	type: T,
	getValues: UseFormGetValues<AddExpenseModalForm>,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm, 'debtors' | 'payers'>,
	debtorsfields: FieldArrayWithId<AddExpenseModalForm>[],
) {
	const debtorsOrPayers = getValues()[type];
	const totalAmountValue = getValues().totalAmount;

	const activeDebtorOrPayersFields = debtorsOrPayers.filter((field) => field.isChecked);
	const activeDebtorOrPayersCount = activeDebtorOrPayersFields.length;

	const updatedAndRoundedDebtorOrPayersAmount = parseFloat((totalAmountValue / activeDebtorOrPayersCount).toFixed(2));

	activeDebtorOrPayersFields.forEach((field) => {
		updateMethod(
			debtorsfields.findIndex((f) => f.user.id === field.user.id),
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
							updateAmounts(type, getValues, updateMethod, fields);
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
