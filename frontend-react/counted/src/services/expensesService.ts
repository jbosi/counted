import { httpClient } from '../shared';
import type { CreatableExpense, Expense } from '../types/expenses.model';

const API_BASE = '/api/expenses';

export const expensesService = {
	async createExpenseAsync(creatableUser: CreatableExpense): Promise<Expense> {
		return httpClient.post(`${API_BASE}`, creatableUser);
	},
};
