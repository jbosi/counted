import { Injectable } from '@angular/core';
import { FetchHttpClient, HttpFetchParams } from '../fetch';
import { ICreatableProject, IProject } from './models';

const API_URL = '/api/projects';

@Injectable({ providedIn: 'root' })
export class ProjectsHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(user_id: number, project_id?: string): Promise<IProject[]> {
		const params: HttpFetchParams = new Map([
			['user_id', user_id]
		])

		if (project_id != null && project_id !== '') {
			params.set('project_id', project_id)
		}

		return this.http.get(API_URL, params);
	}

	public createAsync(candidate: ICreatableProject): Promise<IProject> {
		return this.http.post(API_URL, candidate);
	}
}