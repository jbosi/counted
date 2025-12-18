import { httpClient } from '../shared';
import type { CreatableUser, User } from '../types/users.model';

const API_BASE = '/api/users';

export const usersService = {
	async createUserAsync(creatableUser: CreatableUser): Promise<User> {
		return httpClient.post(`${API_BASE}`, creatableUser);
	},
};
