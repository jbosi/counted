import { httpClient } from '../shared';
import type { Expense } from '../types/expenses.model';
import type { CreatableProject, EditableProject, ProjectDto } from '../types/projects.model';
import type { ProjectSummary } from '../types/summary.model';
import type { User } from '../types/users.model';

const API_BASE = '/api/projects';

export const projectsService = {
	async getUsersByProjectIdAsync(project_id: string | undefined): Promise<User[]> {
		return httpClient.get(`${API_BASE}/${project_id}/users`);
	},

	async getAllAsync(): Promise<ProjectDto[]> {
		// TMP
		return httpClient.get(`${API_BASE}`);
	},

	async getByProjectIds(projectsIds: string[]): Promise<ProjectDto[]> {
		return httpClient.post(`${API_BASE}/batch`, { ids: projectsIds });
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

	async editProjectAsync(editableProject: EditableProject): Promise<ProjectDto> {
		return httpClient.put(`${API_BASE}`, editableProject);
	},

	async deleteProjectAsync(projectId: string): Promise<void> {
		return httpClient.delete(`${API_BASE}/${projectId}`);
	},
};
