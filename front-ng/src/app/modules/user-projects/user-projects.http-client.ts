import { Injectable } from '@angular/core';
import { FetchHttpClient, HttpFetchParams } from '../fetch';
import { ICreatableUserProject, IUserProjectDto } from './models';

const API_URL = '/api/user-projects';

@Injectable({ providedIn: 'root' })
export class UserProjectsHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(user_id: number): Promise<IUserProjectDto[]> {
		const params: HttpFetchParams = new Map([
			['user_id', user_id]
		])
		return this.http.get(API_URL, params);
	}

	public createAsync(candidate: ICreatableUserProject): Promise<IUserProjectDto> {
		return this.http.post(API_URL, candidate);
	}
}