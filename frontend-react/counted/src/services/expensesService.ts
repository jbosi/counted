import { httpClient } from '../shared';
import type { CreatableExpense, EditableExpense, Expense } from '../types/expenses.model';
import type { Payment } from '../types/payments.model';

const API_BASE = '/api/v1/expenses';

export const expensesService = {
	async createExpenseAsync(creatableExpense: CreatableExpense): Promise<Expense> {
		return httpClient.post(`${API_BASE}`, creatableExpense);
	},

	async editExpenseAsync(editableExpense: EditableExpense): Promise<Expense> {
		return httpClient.put(`${API_BASE}`, editableExpense);
	},

	async getExpenseById(expenseId: number): Promise<Expense> {
		return httpClient.get(`${API_BASE}/${expenseId}`);
	},

	async getPaymentsByExpenseId(expenseId: number): Promise<Payment[]> {
		return httpClient.get(`${API_BASE}/${expenseId}/payments`);
	},

	async deleteExpense(expenseId: number): Promise<void> {
		return httpClient.delete(`${API_BASE}/${expenseId}`);
	},
};
