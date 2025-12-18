import type { User } from '../types/users.model';

const API_BASE = '/api/users';

export const usersService = {
	async cretateUserAsync(creatableUser: { name: string; project_id: string }): Promise<User> {
		// TODO find a way not to wrap the payload with its variable ²name
		const res = await fetch(`${API_BASE}`, { body: JSON.stringify({ user: creatableUser }), method: 'POST' });

		if (!res.ok) {
			let message = `Request failed with status ${res.status}`;
			const errorBody: { error: string } = await res.json();
			if (typeof res === 'string' && errorBody) {
				message += `: ${errorBody}`;
			} else if (errorBody && typeof errorBody === 'object' && 'error' in errorBody) {
				message += `: ${errorBody.error}`;
			}

			throw new Error(message);
		}

		return res.json();
	},
};
