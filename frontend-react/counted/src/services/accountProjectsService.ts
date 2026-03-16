import { httpClient } from '../shared';
import type { CountedLocalStorageProject } from '../types/localStorage.model';

const API_BASE = '/api/v1/account/projects';

export const accountProjectsService = {
	async getAll(signal?: AbortSignal): Promise<CountedLocalStorageProject[]> {
		const data: { projectId: string; userId: number | null }[] = await httpClient.get(API_BASE, signal);
		return data.map((d) => ({ projectId: d.projectId, userId: d.userId }));
	},

	async upsert(projectId: string, userId: number | null): Promise<void> {
		await httpClient.post(API_BASE, { projectId, userId });
	},

	async upsertBatch(entries: CountedLocalStorageProject[]): Promise<string[]> {
		if (entries.length === 0) return [];
		return httpClient.post(
			`${API_BASE}/batch`,
			entries.map((e) => ({ projectId: e.projectId, userId: e.userId })),
		);
	},

	async remove(projectId: string): Promise<void> {
		await httpClient.delete(`${API_BASE}/${projectId}`);
	},
};
