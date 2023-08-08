import { Injectable } from '@angular/core';
import { FetchHttpClient } from '../fetch';
import { ICreatableUsers, IUser } from './models';

const API_URL = '/api/users';

@Injectable({ providedIn: 'root' })
export class UsersHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	// TODO: scope to project
	public getAsync(): Promise<IUser[]> {
		return this.http.get(API_URL);
	}

	public async createAsync(candidate: ICreatableUsers): Promise<IUser> {
		return this.http.post(API_URL, candidate);
	}

	public async deleteAsync(userId: number): Promise<void> {
		return this.http.delete(`${API_URL}/${userId}`);
	}

	public async getUsersByProjectIdAsync(projectId: string): Promise<IUser[]> {
		return this.http.get(`${API_URL}/projects/${projectId}`)
	}
}