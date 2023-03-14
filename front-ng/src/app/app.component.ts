import { Component, OnInit } from '@angular/core';
import { IProject, IUser, ProjectsHttpClient, UsersHttpClient } from './modules';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
	public title = 'front-ng';
	public projects: IProject[] = [];
	public users: IUser[] = [];

	constructor(
		private readonly projectHttpClient: ProjectsHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}

	async ngOnInit(): Promise<void> {
		this.projects = await this.projectHttpClient.getAsync();
		this.users = await this.usersHttpClient.getAsync();
	}

	public async addProjectAsync(): Promise<void> {
		this.projectHttpClient.createAsync({ name: 'ProjectAvecUser1et2', users: [1, 2] });
	}
}
