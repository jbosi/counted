import { Injectable } from '@angular/core';
import { FetchHttpClient } from '../fetch';
import { IUser, ICreatableUser } from './models';

const API_URL = '/api/users';

@Injectable({ providedIn: 'root' })
export class UsersHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	// TODO: scope to project
	public getAsync(): Promise<IUser[]> {
		return this.http.get(API_URL);
	}

	public createAsync(candidate: ICreatableUser): Promise<IUser> {
		return this.http.post(API_URL, candidate);
	}
}