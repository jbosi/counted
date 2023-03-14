import { Injectable } from '@angular/core';
import { FetchHttpClient } from '../fetch';
import { ICreatableProject, IProject } from './models';

const API_URL = '/api/projects';

@Injectable({ providedIn: 'root' })
export class ProjectsHttpClient {
	constructor(private http: FetchHttpClient) { }
	
	public getAsync(): Promise<IProject[]> {
		return this.http.get(API_URL);
	}

	public createAsync(candidate: ICreatableProject): Promise<IProject> {
		return this.http.post(API_URL, candidate);
	}
}