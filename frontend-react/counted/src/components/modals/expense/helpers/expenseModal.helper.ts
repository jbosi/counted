import { ExpenseTypeConst, type ExpenseType } from '../../../../types/expenses.model';
import * as z from 'zod';

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
	isChecked: z.boolean(),
	user: expenseUserSchema,
});

export const expenseFormSchema = z.object({
	name: z.string().min(2, 'Le nom doit contenir au moins 2 caractères').max(100, 'Le nom ne doit pas dépasser 100 caractères'),
	description: z.string().max(200, 'La description ne doit pas dépasser 200 caractères').optional(),
	totalAmount: z
		.number({ error: 'Le montant doit être un nombre' })
		.min(0.01, 'Le montant doit être supérieur à 0')
		.max(100000, 'Le montant ne doit pas dépasser 100 000'),
	type: z.enum(ExpenseTypeConst, 'Type de dépense invalide'),
	date: z.string().min(1, 'La date est requise'),
	payers: z.array(expenseCheckboxFormSchema).min(1, 'Au moins un payeur est requis'),
	debtors: z.array(expenseCheckboxFormSchema).min(1, 'Au moins un débiteur est requis'),
});
