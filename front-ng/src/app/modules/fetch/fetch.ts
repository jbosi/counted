/* eslint-disable no-restricted-syntax */
/* eslint-disable arrow-body-style */
import { Injectable } from '@angular/core';
import {
	HttpFetchParams, HttpMethod, IFetchErrorResponse, IFetchRequestOptions
} from './fetch.model';

@Injectable()
export class FetchHttpClient {
	public async get<T>(url: string, params?: HttpFetchParams): Promise<T> {
		return sendRequest<T>(url, 'GET', null, { params });
	}

	// TODO merge options & params
	public async post<T>(url: string, body: any, params?: HttpFetchParams, options?: IFetchRequestOptions): Promise<T> {
		return sendRequest<T>(url, 'POST', body, { params, ...options });
	}

	public async put<T>(url: string, body: any, params?: HttpFetchParams, options?: IFetchRequestOptions): Promise<T> {
		return sendRequest<T>(url, 'PUT', body, { params, ...options });
	}

	public async patch<T>(url: string, body: any, params?: HttpFetchParams): Promise<T> {
		return sendRequest<T>(url, 'PATCH', body, { params });
	}

	public async delete<T>(url: string, params?: HttpFetchParams): Promise<T> {
		return sendRequest<T>(url, 'DELETE', null, { params });
	}
}


const sendRequest = async <T>(
	url: string,
	method: HttpMethod,
	body?: any,
	options?: IFetchRequestOptions
): Promise<T> => {

	const timeout = getTimeout(options);

	let fetchBody: BodyInit | undefined;
	let headers: HeadersInit | undefined;
	const isFile = body instanceof FormData;
	if (isFile) {
		fetchBody = body;
	} else {
		fetchBody = body ? JSON.stringify(body) : undefined;
		headers = { 'Content-Type': 'application/json;charset=UTF-8' };
	}

	const fetchOptions: RequestInit = {
		method,
		headers,
		signal: timeout.signal,
		body: fetchBody,
	};

	const urlWithParams = getUrlWithParams(url, options);

	return fetch(urlWithParams, fetchOptions)
		.then(async r => {
			if (!r.ok) {
				throw await handleFetchError(r);
			}
			if (isFile) {
				const contentLength = r.headers.get('content-length');
				if (!contentLength) {
					throw Error('Content-Length response header unavailable');
				}
				return r.json();
			}
			const val = await r.text();
			if (val == null || val === '') {
				return;
			}
			try {
				return JSON.parse(val);
			} catch (e) {
				throw new Error(`Unable to parse ${val}`);
			}
		});
};

const handleFetchError = async (response: Response): Promise<Error> => {
	return response.json().then((error: (IFetchErrorResponse)) => {
		return Error('ERROR_OCCURRED');
	});
};


/** Add timeout if any */
const getTimeout = (options?: IFetchRequestOptions): AbortController => {
	const timeoutController = new AbortController();
	const timeout = options?.timeout ?? 15000;

	setTimeout(() => timeoutController.abort(), timeout);

	return timeoutController;
};

/** Append http params to url if any */
const getUrlWithParams = (url: string, options?: IFetchRequestOptions): string => {
	let urlWithParams = url;

	const params: HttpFetchParams = !!options?.params ? options.params : undefined;

	if (params?.size == null) {
		return urlWithParams;
	}

	if (params?.size > 0) {
		if (url.indexOf('?') === -1) {
			urlWithParams += '?';
		}
		params?.forEach((param, key) => {
			urlWithParams += `${key}=${param}&`;
		});
		urlWithParams = urlWithParams.slice(0, urlWithParams.length - 1);
	}

	return urlWithParams;
};
