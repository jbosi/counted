import { NgFor } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { IProject, IUser, ProjectsHttpClient, UsersHttpClient } from '../../modules';
import { AddProjectModalComponent } from './components/add-project-modal/add-project-modal.component';
import { CardComponent } from './components/card/card.component';

@Component({
	selector: 'app-projects',
	templateUrl: './projects.component.html',
	styleUrls: ['./projects.component.scss'],
	standalone: true,
	imports: [NgFor, CardComponent, AddProjectModalComponent]
})
export class ProjectsComponent implements OnInit {
	public projects: IProjectCardViewModel[] = [];
	public users: IUser[] = [];

	constructor(
		private readonly projectsHttpClient: ProjectsHttpClient,
		private readonly usersHttpClient: UsersHttpClient
	) {}

	async ngOnInit(): Promise<void> {
		await this.getData()
	}

	async onProjectAddedAsync(): Promise<void> {
		await this.getData()
	}

	public async addProjectAsync(): Promise<void> {
		await this.projectsHttpClient.createAsync({ name: 'ProjectAvecUser1', users: [1] });
		await this.getData()
	}

	private async getData(): Promise<void> {
		this.users = await this.usersHttpClient.getAsync();
		const projects: IProject[] = await this.projectsHttpClient.getAsync(1);
		
		this.projects = projects.map(project => {
			const users = this.users.filter(user => project.users.includes(user.id))
			return { ...project, users  }
		});
	}
}

export interface IProjectCardViewModel extends Omit<IProject, 'users'> { // TODO change when userId
	users: IUser[]
}