import { httpClient } from '../shared';
import type { CreatableExpense, Expense } from '../types/expenses.model';
import type { Payment } from '../types/payments.model';

const API_BASE = '/api/expenses';

export const expensesService = {
	async createExpenseAsync(creatableUser: CreatableExpense): Promise<Expense> {
		return httpClient.post(`${API_BASE}`, creatableUser);
	},

	async getPaymentsByExpenseId(expenseId: number): Promise<Payment[]> {
		return httpClient.get(`${API_BASE}/${expenseId}/payments`);
	},
};
