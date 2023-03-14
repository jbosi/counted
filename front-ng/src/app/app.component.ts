import { Component, OnInit } from '@angular/core';
import { IProject, IUser, ProjectsHttpClient, UsersHttpClient } from './modules';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
	public title = 'front-ng';
	public projects: IProjectCardViewModel[] = [];
	public users: IUser[] = [];

	constructor(
		private readonly projectHttpClient: ProjectsHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}

	async ngOnInit(): Promise<void> {
		await this.getData()
	}

	async onProjectAddedAsync(): Promise<void> {
		await this.getData()
	}

	public async addProjectAsync(): Promise<void> {
		await this.projectHttpClient.createAsync({ name: 'ProjectAvecUser1', users: [1] });
		await this.getData()
	}

	private async getData(): Promise<void> {
		this.users = await this.usersHttpClient.getAsync();
		const projects: IProject[] = await this.projectHttpClient.getAsync();
		
		this.projects = projects.map(project => {
			const users = this.users.filter(user => project.users.includes(user.id))
			return { ...project, users  }
		});
	}
}

export interface IProjectCardViewModel extends Omit<IProject, 'users'> { // TODO change when userId
	users: IUser[]
}