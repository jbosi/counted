import type { FormEvent, RefObject } from 'react';
import { type FieldArrayWithId, type FormState, type SubmitHandler, type UseFieldArrayUpdate, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import type { CreatableProject, EditableProject, ProjectDto } from '../../../types/projects.model';
import type { UseMutationResult } from '@tanstack/react-query';
import type { User } from '../../../types/users.model';
import { ErrorValidationCallout } from '../../errorCallout';
import type { AddExpenseModalForm } from './addExpenseModal';

export interface ExpenseModalContentProps {
	isEdition: boolean;
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	register: UseFormRegister<AddExpenseModalForm>;
	onSubmit: SubmitHandler<AddExpenseModalForm | EditExpenseModalForm>;
	getValues: UseFormGetValues<AddExpenseModalForm | EditExpenseModalForm>;
	formState: FormState<AddExpenseModalForm | EditExpenseModalForm>;
	mutationHook: UseMutationResult<ProjectDto, Error, CreatableProject, unknown> | UseMutationResult<ProjectDto, Error, EditableProject, unknown>;
	payersFields: FieldArrayWithId<AddExpenseModalForm, 'payers', 'id'>[] | FieldArrayWithId<EditExpenseModalForm, 'payers', 'id'>[];
	debtorsFields: FieldArrayWithId<AddExpenseModalForm, 'debtors', 'id'>[] | FieldArrayWithId<EditExpenseModalForm, 'debtors', 'id'>[];
	updatePayer: UseFieldArrayUpdate<AddExpenseModalForm, 'payers'> | UseFieldArrayUpdate<EditExpenseModalForm | 'payers'>;
	updateDebtor: UseFieldArrayUpdate<AddExpenseModalForm, 'debtors'> | UseFieldArrayUpdate<EditExpenseModalForm | 'debtors'>;
	errorState: string | null;
	handleSubmit: (e: FormEvent<HTMLFormElement>) => void;
	exitModal: () => void;
}

export function ExpenseModalContent({
	isEdition,
	dialogRef,
	modalId,
	onSubmit,
	getValues,
	register,
	formState,
	mutationHook,
	payersFields,
	debtorsFields,
	updatePayer,
	updateDebtor,
	errorState,
	handleSubmit,
	exitModal,
}: ExpenseModalContentProps) {
	const errors = formState.errors;
	const { error, isPending, isError } = mutationHook;

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
										updateAmounts('debtors', getValues, updateDebtor, debtorsFields);
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
								{debtorsFields.map((field, index) => (
									<FormCheckbox
										key={field.id}
										amount={field.amount}
										isChecked={field.isChecked}
										user={field.user}
										index={index}
										register={register}
										getValues={getValues}
										updateMethod={updateDebtor}
										fields={debtorsFields}
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
	const amountRemaining = totalAmountValue - updatedAndRoundedDebtorOrPayersAmount * activeDebtorOrPayersCount;
	const updatedDebtorOrPayersAmountWithRemain = updatedAndRoundedDebtorOrPayersAmount + amountRemaining;

	activeDebtorOrPayersFields.forEach((field, index) => {
		const isLast = activeDebtorOrPayersCount === index + 1;
		updateMethod(
			debtorsfields.findIndex((f) => f.user.id === field.user.id),
			{ amount: isLast ? updatedDebtorOrPayersAmountWithRemain : updatedAndRoundedDebtorOrPayersAmount, isChecked: field.isChecked, user: field.user },
		);
	});
}

interface FormCheckboxProps {
	user: User;
	register: UseFormRegister<AddExpenseModalForm> | UseFormRegister<EditExpenseModalForm>;
	getValues: UseFormGetValues<AddExpenseModalForm> | UseFormGetValues<EditExpenseModalForm>;
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm> | UseFieldArrayUpdate<EditExpenseModalForm>;
	fields: FieldArrayWithId<AddExpenseModalForm>[] | FieldArrayWithId<EditExpenseModalForm>[];
	type: 'debtors' | 'payers';
	amount: number;
	isChecked: boolean;
	index: number;
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
