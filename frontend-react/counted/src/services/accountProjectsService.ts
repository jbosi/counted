import { httpClient } from '../shared';
import type { CountedLocalStorageProject } from '../types/localStorage.model';

const API_BASE = '/api/v1/account/projects';

export const accountProjectsService = {
	async getAll(): Promise<CountedLocalStorageProject[]> {
		const data: { projectId: string; userId: number | null }[] = await httpClient.get(API_BASE);
		return data.map((d) => ({ projectId: d.projectId, userId: d.userId }));
	},

	async upsert(projectId: string, userId: number | null): Promise<void> {
		await httpClient.post(API_BASE, { projectId, userId });
	},

	async remove(projectId: string): Promise<void> {
		await httpClient.delete(`${API_BASE}/${projectId}`);
	},
};
