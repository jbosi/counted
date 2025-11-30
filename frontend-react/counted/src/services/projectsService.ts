import type { ProjectDto } from '../types/project.model';
import type { User } from '../types/users.model';

const API_BASE = 'http://127.0.0.1:53627/api/projects';

export const projectsService = {
	async getUsersByProjectIdAsync(project_id: string): Promise<User[]> {
		const res = await fetch(`${API_BASE}/${project_id}/users`);

		if (!res.ok) {
			throw new Error('Error while fetching films');
		}

		return res.json();
	},

	async getAllAsync(): Promise<ProjectDto[]> {
		const res = await fetch(`${API_BASE}`);

		if (!res.ok) {
			throw new Error('Error while fetching films');
		}

		return res.json();
	},
};
