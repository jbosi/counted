import type { FormCheckboxGroupProps } from '../addExpenseModal';

type ExpenseShareInputProps = Omit<FormCheckboxGroupProps, 'isChecked' | 'fields' | 'shareMode'>;

export function ExpenseShareInput({ amount, register, type, user, index, getValues, updateMethod, onRecalculate }: ExpenseShareInputProps) {
	return (
		<div className="flex items-center gap-2">
			<span className="text-sm text-base-content/60 min-w-14 text-right">{amount.toFixed(2)} €</span>
			<input
				className="input w-20"
				type="number"
				min="0"
				step="1"
				{...register(`${type}.${index}.shares`, {
					valueAsNumber: true,
					onBlur() {
						const shares = getValues(`${type}.${index}.shares`) ?? 0;
						if (shares > 0 && !getValues(`${type}.${index}.isChecked`)) {
							updateMethod(index, { amount: 0, isChecked: true, user, shares });
						}
						onRecalculate?.();
					},
				})}
			/>
			<span className="text-xs opacity-60">part(s)</span>
		</div>
	);
}
