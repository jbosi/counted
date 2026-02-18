import { httpClient } from '../shared';
import type { CreatableUser, User } from '../types/users.model';

const API_BASE = '/api/v1/users';

export const usersService = {
	async createUsersAsync(creatableUsers: CreatableUser[]): Promise<User[]> {
		return httpClient.post(`${API_BASE}`, creatableUsers);
	},

	async deleteUserAsync(userId: number): Promise<User> {
		return httpClient.delete(`${API_BASE}/${userId}`);
	},
};
