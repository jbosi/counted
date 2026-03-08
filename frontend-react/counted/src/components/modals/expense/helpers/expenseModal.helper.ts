import type { FieldArrayWithId, UseFieldArrayUpdate, UseFormGetValues } from 'react-hook-form';
import * as z from 'zod';
import { ExpenseTypeConst, type ExpenseType } from '../../../../types/expenses.model';
import type { User } from '../../../../types/users.model';
import type { AddExpenseModalForm } from '../addExpenseModal';

export const getPayersFieldLabel = (expenseType: ExpenseType) => {
	switch (expenseType) {
		case 'Gain':
			return 'Qui a reçu l`argent ?';
		case 'Transfer':
			return 'Qui transfère l`argent ?';
		case 'Expense':
		default:
			return 'Qui a payé ?';
	}
};

export const getDebtorsFieldLabel = (expenseType: ExpenseType) => {
	switch (expenseType) {
		case 'Gain':
			return 'Pour qui ?';
		case 'Transfer':
			return 'Vers qui ?';
		case 'Expense':
		default:
			return 'Pour qui ?';
	}
};

export const expenseUserSchema = z.object({
	id: z.number(),
	name: z.string(),
	balance: z.number().nullish(),
	created_at: z.string().nullish(),
});

export const expenseCheckboxFormSchema = z.object({
	amount: z.number({ error: 'Le montant doit être un nombre' }).min(0, 'Le montant doit être positif'),
	shares: z.number().int().min(0),
	isChecked: z.boolean(),
	user: expenseUserSchema,
});

export const expenseFormSchema = z.object({
	name: z.string().min(2, 'Le nom doit contenir au moins 2 caractères').max(100, 'Le nom ne doit pas dépasser 100 caractères'),
	totalAmount: z
		.number({ error: 'Le montant doit être un nombre' })
		.min(0.01, 'Le montant doit être supérieur à 0')
		.max(100000, 'Le montant ne doit pas dépasser 100 000'),
	type: z.enum(ExpenseTypeConst, 'Type de dépense invalide'),
	date: z.string().min(1, 'La date est requise'),
	payers: z.array(expenseCheckboxFormSchema).min(1, 'Au moins un payeur est requis'),
	debtors: z.array(expenseCheckboxFormSchema).min(1, 'Au moins un débiteur est requis'),
});

export function resetExpenseAmountOnUnchecked(
	getValues: UseFormGetValues<AddExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>,
	user: User,
) {
	const isChecked = getValues(`${type}.${index}.isChecked`);
	if (!isChecked) {
		updateMethod(index, { amount: 0, isChecked, user, shares: getValues(`${type}.${index}.shares`) ?? 0 });
	}
}

export function toggleCheckedIfAmountChange(
	getValues: UseFormGetValues<AddExpenseModalForm>,
	type: 'debtors' | 'payers',
	index: number,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm>,
	user: User,
) {
	const amount = getValues(`${type}.${index}.amount`);
	const isChecked = getValues(`${type}.${index}.isChecked`);
	const shares = getValues(`${type}.${index}.shares`) ?? 0;
	if (amount > 0 && !isChecked) {
		updateMethod(index, { amount, isChecked: true, user, shares });
	} else if (amount === 0 && isChecked) {
		updateMethod(index, { amount, isChecked: false, user, shares });
	}
}

export function updateAmounts<T extends 'debtors' | 'payers'>(
	type: T,
	values: AddExpenseModalForm,
	updateMethod: UseFieldArrayUpdate<AddExpenseModalForm, 'debtors' | 'payers'>,
	debtorsOrPayersfields: FieldArrayWithId<AddExpenseModalForm>[],
	shareMode?: boolean,
) {
	const debtorsOrPayers = values[type];
	const totalAmountValue = values.totalAmount;

	const activeDebtorOrPayersFields = debtorsOrPayers.filter((field) => field.isChecked);
	const activeDebtorOrPayersCount = activeDebtorOrPayersFields.length;

	if (activeDebtorOrPayersCount === 0) {
		return;
	}

	if (shareMode) {
		const totalShares = activeDebtorOrPayersFields.reduce((sum, f) => sum + (f.shares ?? 1), 0);
		if (totalShares === 0) {
			return;
		}
		activeDebtorOrPayersFields.forEach((field) => {
			const shares = field.shares ?? 1;
			const amount = parseFloat(((shares / totalShares) * totalAmountValue).toFixed(2));
			updateMethod(
				debtorsOrPayersfields.findIndex((f) => f.user.id === field.user.id),
				{ amount, isChecked: field.isChecked, user: field.user, shares },
			);
		});
	} else {
		const updatedAndRoundedDebtorOrPayersAmount = parseFloat((totalAmountValue / activeDebtorOrPayersCount).toFixed(2));
		activeDebtorOrPayersFields.forEach((field) => {
			updateMethod(
				debtorsOrPayersfields.findIndex((f) => f.user.id === field.user.id),
				{ amount: updatedAndRoundedDebtorOrPayersAmount, isChecked: field.isChecked, user: field.user, shares: field.shares ?? 0 },
			);
		});
	}
}
