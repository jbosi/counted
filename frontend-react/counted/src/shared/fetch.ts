export const httpClient = {
	async get(url: string) {
		return sendRequest(url, 'GET');
	},

	async put(url: string, payload: unknown) {
		return sendRequest(url, 'PUT', payload);
	},

	async post(url: string, payload: unknown) {
		return sendRequest(url, 'POST', payload);
	},

	async patch(url: string, payload: unknown) {
		return sendRequest(url, 'PATCH', payload);
	},

	async delete(url: string, payload?: unknown) {
		return sendRequest(url, 'DELETE', payload);
	},
};

const sendRequest = async (url: string, method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH', payload?: unknown) => {
	const res = await fetch(url, { body: JSON.stringify(payload), method, headers: [['Content-Type', 'application/json']] });

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
};
