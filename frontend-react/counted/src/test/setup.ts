import { expect, afterEach, beforeAll } from 'vitest';
import { cleanup } from '@testing-library/react';

// Mock localStorage
const localStorageMock = (() => {
	let store: Record<string, string> = {};

	return {
		getItem: (key: string) => store[key] || null,
		setItem: (key: string, value: string) => {
			store[key] = value.toString();
		},
		removeItem: (key: string) => {
			delete store[key];
		},
		clear: () => {
			store = {};
		},
	};
})();

beforeAll(() => {
	Object.defineProperty(globalThis, 'localStorage', {
		value: localStorageMock,
		writable: true,
	});
});

// Cleanup after each test
afterEach(() => {
	cleanup();
	localStorageMock.clear();
});
