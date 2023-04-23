import { Injectable } from '@angular/core';
import { FetchHttpClient } from '../fetch';
import { ICreatableExpense, IExpense } from './models';

const API_URL = '/api/projects';

@Injectable({ providedIn: 'root' })
export class ExpensesHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(projectId: string): Promise<IExpense[]> {
		return this.http.get(`${API_URL}/${projectId}/expenses`);
	}

	public createAsync(projectId: string, candidate: ICreatableExpense): Promise<IExpense> {
		return this.http.post(`${API_URL}/${projectId}/expenses`, candidate);
	}
}