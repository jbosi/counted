import { Component, OnInit } from '@angular/core';
import { IProject } from './modules';
import { ProjectsHttpClient } from './modules/projects/projects.http-client';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
	public title = 'front-ng';
	public projects: IProject[] = [];

	constructor(private readonly projectHttpClient: ProjectsHttpClient) {}

	async ngOnInit(): Promise<void> {
		this.projects = await this.projectHttpClient.getAsync();
	}

	public async addProjectAsync(): Promise<void> {
		this.projectHttpClient.createAsync({ name: 'ProjectAvecUser1et2', users: [1, 2] });
	}
}
