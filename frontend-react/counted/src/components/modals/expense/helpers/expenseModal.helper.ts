import type { ExpenseType } from '../../../../types/expenses.model';

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
			return 'Qui doit rembourser ?';
	}
};
