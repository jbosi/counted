import { Injectable } from '@angular/core';
import { FetchHttpClient } from '../../fetch';
import { IExpensePayments } from '../models';

const API_URL = '/api/projects';

@Injectable({ providedIn: 'root' })
export class ExpensePaymentsHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(projectId: string): Promise<IExpensePayments[]> {
		return this.http.get(`${API_URL}/${projectId}/expensepayments`);
	}
}