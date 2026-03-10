import { httpClient } from '../shared';
import type { Account, LoginPayload, RegisterPayload } from '../types/auth.model';

const API_BASE = '/api/v1/auth';

export const authService = {
	async register(payload: RegisterPayload): Promise<Account> {
		return httpClient.post(`${API_BASE}/register`, payload);
	},

	async login(payload: LoginPayload): Promise<Account> {
		return httpClient.post(`${API_BASE}/login`, payload);
	},

	async logout(): Promise<void> {
		const res = await fetch(`${API_BASE}/logout`, {
			method: 'POST',
			headers: [['Content-Type', 'application/json']],
		});
		if (!res.ok) throw new Error('Logout failed');
	},

	async me(): Promise<Account | null> {
		return httpClient.get(`${API_BASE}/me`);
	},
};
