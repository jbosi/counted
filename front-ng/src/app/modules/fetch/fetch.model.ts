export type HttpMethod = 'POST' | 'GET' | 'PUT' | 'PATCH' | 'DELETE' | 'UPDATE';

export type HttpFetchParams = Map<string, string | number | boolean> | undefined;

export interface IFetchRequestOptions {
	params?: HttpFetchParams;
	timeout?: number;
	reportProgress?: boolean; // TODO
};

export interface IFetchErrorResponse {}

