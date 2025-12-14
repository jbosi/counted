import type { Expense } from '../types/expenses.model';
import type { ProjectDto } from '../types/projects.model';
import type { ProjectSummary } from '../types/summary.model';
import type { User } from '../types/users.model';

const API_BASE = '/api/projects';

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

	async getByProjectId(projectId: string): Promise<ProjectDto> {
		const res = await fetch(`${API_BASE}/${projectId}`);

		if (!res.ok) {
			throw new Error('Error while fetching films');
		}

		return res.json();
	},

	async getExpensesByProjectId(projectId: string): Promise<Expense[]> {
		const res = await fetch(`${API_BASE}/${projectId}/expenses`);

		if (!res.ok) {
			throw new Error('Error while fetching films');
		}

		return res.json();
	},

	async getExpensesSummaryByProjectId(projectId: string): Promise<ProjectSummary> {
		const res = await fetch(`${API_BASE}/${projectId}/expenses/summary`);

		if (!res.ok) {
			throw new Error('Error while fetching films');
		}

		return res.json();
	},
};
