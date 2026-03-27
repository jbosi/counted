import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useMemo, useState, type RefObject } from 'react';
import {
	useFieldArray,
	useForm,
	useWatch,
	type Control,
	type FieldArrayWithId,
	type UseFieldArrayUpdate,
	type UseFormGetValues,
	type UseFormRegister,
} from 'react-hook-form';
import * as z from 'zod';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { useAddExpense } from '../../../hooks/useExpenses';
import { type CreatableExpense, type ExpenseType } from '../../../types/expenses.model';
import type { User } from '../../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import { ExpenseShareInput } from './components/expenseShareInput';
import { SelectAllCheckboxInput } from './components/selectAllCheckboxInput';
import {
	expenseFormSchema,
	getDebtorsFieldLabel,
	getPayersFieldLabel,
	resetExpenseAmountOnUnchecked,
	toggleCheckedIfAmountChange,
	updateAmounts,
} from './helpers/expenseModal.helper';
import { ModalFooter } from '../shared/modalFooter';
import { getPickerFormattedDate } from '../../../utils/date';

export interface AddExpenseModalProps {
	modalId: string;
	projectId: string;
	users: User[];
	dialogRef: RefObject<HTMLDialogElement | null>;
	closeDialogFn: () => void;
	restrictToTransfer?: boolean;
	initialValues?: Partial<AddExpenseModalForm>;
}

export type AddExpenseModalForm = z.infer<typeof expenseFormSchema>;
type FormCheckbox = AddExpenseModalForm['payers'][number];

export interface FormCheckboxGroupProps {
	user: User;
	register: UseFormRegister<AddExpenseModalForm>;
	getValues: UseFormGetValues<AddExpenseModalForm>;
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>;
	fields: FieldArrayWithId<AddExpenseModalForm>[];
	control: Control<AddExpenseModalForm>;
	type: 'debtors' | 'payers';
	isChecked: boolean;
	index: number;
	shareMode?: boolean;
	onRecalculate?: () => void;
}

function getInitialValues(users: User[]): Partial<AddExpenseModalForm> {
	const initialDebtorsFormCheckBoxValues: FormCheckbox[] = users.map((u) => ({
		amount: 0,
		shares: 1,
		isChecked: true,
		user: u,
	}));

	const initialPayersFormCheckBoxValues: FormCheckbox[] = users.map((u) => ({
		amount: 0,
		shares: 0,
		isChecked: false,
		user: u,
	}));

	return {
		payers: initialPayersFormCheckBoxValues,
		debtors: initialDebtorsFormCheckBoxValues,
		totalAmount: 0,
		name: '',
		date: getPickerFormattedDate(new Date()),
	};
}

export function AddExpenseModal({ dialogRef, modalId, users, projectId, closeDialogFn, restrictToTransfer, initialValues }: AddExpenseModalProps) {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const defaultValues = useMemo(
		() => ({ ...getInitialValues(users), ...(restrictToTransfer ? { type: 'Transfer' as ExpenseType } : {}), ...initialValues }),
		[users, restrictToTransfer, initialValues],
	);

	const {
		register,
		handleSubmit,
		formState: { errors },
		getValues,
		setValue,
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

	const handlePayersRecalculate = useCallback(() => {
		updateAmounts('payers', getValues(), setValue, payersFields, payersShareMode);
	}, [getValues, setValue, payersFields, payersShareMode]);

	const handleDebtorsRecalculate = useCallback(() => {
		updateAmounts('debtors', getValues(), setValue, debtorsfields, debtorsShareMode);
	}, [getValues, setValue, debtorsfields, debtorsShareMode]);

	const togglePayersShareMode = useCallback(() => {
		const newMode = !payersShareMode;
		setPayersShareMode(newMode);
		updateAmounts('payers', getValues(), setValue, payersFields, newMode);
	}, [payersShareMode, getValues, setValue, payersFields]);

	const toggleDebtorsShareMode = useCallback(() => {
		const newMode = !debtorsShareMode;
		setDebtorsShareMode(newMode);
		updateAmounts('debtors', getValues(), setValue, debtorsfields, newMode);
	}, [debtorsShareMode, getValues, setValue, debtorsfields]);

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
		<dialog ref={dialogRef} id={modalId} className="modal">
			<div className="modal-box flex gap-3 flex-col">
				<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={closeDialogFn}>
					✕
				</button>
				<h1>{restrictToTransfer ? 'Ajouter un transfert' : 'Ajouter une dépense'}</h1>
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
									updateAmounts('debtors', getValues(), setValue, debtorsfields, debtorsShareMode);
									updateAmounts('payers', getValues(), setValue, payersFields, payersShareMode);
								},
							})}
						/>

						{!restrictToTransfer && (
							<>
								<label className="label">Type de dépense</label>
								<select defaultValue="Dépense" className="select w-full" {...register('type')}>
									<option value={'Expense' as ExpenseType}>Dépense</option>
									<option value={'Gain' as ExpenseType}>Gain</option>
									<option value={'Transfer' as ExpenseType}>Transfert d'argent</option>
								</select>
							</>
						)}
						{restrictToTransfer && <input type="hidden" {...register('type')} value="Transfer" />}

						<fieldset className="fieldset bg-base-100 border-base-300 rounded-box border p-4 w-full">
							<legend className="fieldset-legend">{getPayersFieldLabel(expenseType)}</legend>
							<label className="label justify-end gap-2 mb-1">
								<span className="text-xs">Par parts</span>
								<input type="checkbox" className="toggle toggle-sm" checked={payersShareMode} onChange={togglePayersShareMode} />
							</label>
							<SelectAllCheckboxInput initialValue={false} fields={payersFields} updateMethod={updatePayer} getValues={getValues} type={'payers'} />
							{payersFields.map((field, index) => (
								<FormCheckboxGroup
									key={field.id}
									isChecked={field.isChecked}
									user={field.user}
									index={index}
									register={register}
									getValues={getValues}
									updateMethod={updatePayer}
									fields={payersFields}
									control={control}
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
								<FormCheckboxGroup
									key={field.id}
									isChecked={field.isChecked}
									user={field.user}
									index={index}
									register={register}
									getValues={getValues}
									updateMethod={updateDebtor}
									fields={debtorsfields}
									control={control}
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
	);
}

export function FormCheckboxGroup({ isChecked, register, type, user, index, getValues, updateMethod, control, shareMode, onRecalculate }: FormCheckboxGroupProps) {
	return (
		<div className="label justify-between">
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
								onRecalculate?.();
							}
						},
					})}
				/>
				{user.name}
			</div>
			{shareMode ? (
				<ExpenseShareInput
					control={control}
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
		</div>
	);
}
