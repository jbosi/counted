import { httpClient } from '../shared';
import type { CreatableUser, User } from '../types/users.model';

const API_BASE = '/api/users';

export const usersService = {
	async createUserAsync(creatableUser: CreatableUser): Promise<User> {
		return httpClient.post(`${API_BASE}`, creatableUser);
	},

	async deleteUserAsync(userId: number): Promise<User> {
		return httpClient.delete(`${API_BASE}/${userId}`);
	},
};
