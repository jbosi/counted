import { Injectable } from '@angular/core';
import { FetchHttpClient, HttpFetchParams } from '../fetch';
import { ICreatableExpense, IExpenseDto } from './models';

const API_URL = '/api/expenses';

@Injectable({ providedIn: 'root' })
export class ExpensesHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(projectId: string): Promise<IExpenseDto[]> {
		const params: HttpFetchParams = new Map([
			['project_id', projectId]
		])
		return this.http.get(`${API_URL}`, params);
	}

	public getByIdAsync(expenseId: number): Promise<IExpenseDto> {
		return this.http.get(`${API_URL}/${expenseId}`);
	}

	public createAsync(candidate: ICreatableExpense): Promise<IExpenseDto> {
		return this.http.post(`${API_URL}`, candidate);
	}

	public deleteAsync(expenseId: number): Promise<void> {
		return this.http.delete(`${API_URL}/${expenseId}`);
	}
}