import { Injectable, inject } from '@angular/core';
import { FetchHttpClient } from '../../fetch';
import { IExpensePayments } from '../models';

const API_URL = '/api/projects';

@Injectable({ providedIn: 'root' })
export class ExpensePaymentsHttpClient {
	private http = inject(FetchHttpClient);

	
	public getAsync(projectId: string): Promise<IExpensePayments[]> {
		return this.http.get(`${API_URL}/${projectId}/expensepayments`);
	}

	public async getByIdAsync(projectId: string, expensePaymentId: number): Promise<IExpensePayments | undefined> {
		const expenses = await this.getAsync(projectId);
		return expenses.find(e => e.id === expensePaymentId);
	}
}