import { httpClient } from '../shared';
import type { Expense } from '../types/expenses.model';
import type { CreatableProject, ProjectDto } from '../types/projects.model';
import type { ProjectSummary } from '../types/summary.model';
import type { User } from '../types/users.model';

const API_BASE = '/api/projects';

export const projectsService = {
	async getUsersByProjectIdAsync(project_id: string): Promise<User[]> {
		return httpClient.get(`${API_BASE}/${project_id}/users`);
	},

	async getAllAsync(): Promise<ProjectDto[]> {
		return httpClient.get(`${API_BASE}`);
	},

	async getByProjectId(projectId: string): Promise<ProjectDto> {
		return httpClient.get(`${API_BASE}/${projectId}`);
	},

	async getExpensesByProjectId(projectId: string): Promise<Expense[]> {
		return httpClient.get(`${API_BASE}/${projectId}/expenses`);
	},

	async getExpensesSummaryByProjectId(projectId: string): Promise<ProjectSummary> {
		return httpClient.get(`${API_BASE}/${projectId}/expenses/summary`);
	},

	async createProjectAsync(creatableProject: CreatableProject): Promise<ProjectDto> {
		return httpClient.post(`${API_BASE}`, creatableProject);
	},

	async deleteProjectAsync(projectId: string): Promise<void> {
		return httpClient.delete(`${API_BASE}/${projectId}`);
	},
};
