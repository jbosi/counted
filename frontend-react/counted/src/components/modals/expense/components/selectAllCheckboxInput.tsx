import type { FieldArrayWithId, UseFieldArrayUpdate, UseFormGetValues } from 'react-hook-form';
import type { AddExpenseModalForm } from '../addExpenseModal';
import { resetExpenseAmountOnUnchecked, updateAmounts } from '../helpers/expenseModal.helper';

interface SelectAllCheckboxInputProps<T extends 'debtors' | 'payers' = 'debtors' | 'payers'> {
	type: T;
	fields: FieldArrayWithId<AddExpenseModalForm>[];
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm, T>;
	getValues: UseFormGetValues<AddExpenseModalForm>;
	initialValue: boolean;
}

export function SelectAllCheckboxInput({ type, fields, updateMethod, getValues, initialValue }: SelectAllCheckboxInputProps) {
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
