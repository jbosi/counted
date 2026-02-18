import { httpClient } from '../shared';
import type { ProjectDto } from '../types/projects.model';
import type { User } from '../types/users.model';

export interface TricountImportRequest {
	tricountKey: string;
}

export interface TricountImportResponse {
	project: ProjectDto;
	users: User[];
	expensesCount: number;
}

export const importService = {
	async importTricount(request: TricountImportRequest): Promise<TricountImportResponse> {
		return httpClient.post('/api/import/tricount', request);
	},
};
