import type { RefObject } from 'react';
import type { User } from '../../types/users.model';
import { useFieldArray, useForm, type UseFormRegister } from 'react-hook-form';
import { useAddExpense } from '../../hooks/useExpenses';
import type { CreatableExpense, ExpenseType } from '../../types/expenses.model';

export interface AddExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
}

interface AddExpenseModalForm {
	name: string;
	description: string;
	totalAmount: number;
	type: ExpenseType;
	payers: FormCheckbox[];
	debtors: FormCheckbox[];
}

interface FormCheckboxProps {
	user: User;
	register: UseFormRegister<AddExpenseModalForm>;
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

export function AddExpenseModal({ dialogRef, modalId, users, projectId }: AddExpenseModalProps) {
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

	const {
		register,
		formState: { errors },
		getValues,
		control,
	} = useForm<AddExpenseModalForm>({
		defaultValues: {
			payers: initialPayersFormCheckBoxValues,
			debtors: initialDebtorsFormCheckBoxValues,
		},
	});

	const { fields: payersFields } = useFieldArray({ control, name: 'payers' });
	const { fields: debtorsfields } = useFieldArray({ control, name: 'debtors' });

	const { mutate, isPending, isError, error } = useAddExpense();

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>Ajouter une dépense</h1>
					<form
						className="ml-4 mr-4"
						onSubmit={(e) => {
							e.preventDefault();

							const formValues = getValues();
							console.log(formValues);
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
						}}
					>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('name', { required: true, maxLength: 100 })} />
							{errors.name && <span>Ce champ est requis</span>}

							<label className="label">Description</label>
							<input className="input w-full" {...register('description', { maxLength: 200 })} />

							<label className="label">Valeur</label>
							<input min="0" className="input w-full" type="number" {...register('totalAmount', { required: true, valueAsNumber: true })} />

							<label className="label">Type de dépense</label>
							<select defaultValue="Dépense" className="select w-full" {...register('type', { required: true })}>
								<option value={'Expense'}>Dépense</option>
								<option value={'Gain'}>Gain</option>
								<option value={'Transfert'}>Transfert d'argent</option>
							</select>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">Qui a payé ?</legend>
								{payersFields.map((field, index) => (
									<FormCheckbox key={field.id} amount={field.amount} isChecked={field.isChecked} user={field.user} index={index} register={register} type="payers" />
								))}
							</fieldset>

							<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
								<legend className="fieldset-legend">Qui doit rembourser ?</legend>
								{debtorsfields.map((field, index) => (
									<FormCheckbox key={field.id} amount={field.amount} isChecked={field.isChecked} user={field.user} index={index} register={register} type="debtors" />
								))}
							</fieldset>

							{isPending && <span className="loading loading-spinner loading-xs"></span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className="btn btn-primary" type="submit">
								Enregistrer
							</button>
							<button className="btn btn-outline" type="button" onClick={() => dialogRef.current?.close()}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}

export function FormCheckbox({ isChecked, register, type, user, index }: FormCheckboxProps) {
	return (
		<label className="label justify-between">
			<div className="flex gap-2">
				<input type="checkbox" defaultChecked={isChecked ?? undefined} className="checkbox" {...register(`${type}.${index}.isChecked`)} />
				{user.name}
			</div>
			<input min="0" className="input w-44" type="number" {...register(`${type}.${index}.amount`, { valueAsNumber: true })} />
		</label>
	);
}
