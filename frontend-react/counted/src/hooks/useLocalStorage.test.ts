import { act, renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { type CountedLocalStorage, type CountedLocalStorageProject, COUNTED_LOCAL_STORAGE_KEY } from '../types/localStorage.model';
import { addToLocalStorage, computeProjectsToSync, removeFromLocalStorage, saveProjectEntry, useAddToLocalStorage, useInitializeLocalStorage } from './useLocalStorage';

describe('useLocalStorage', () => {
	beforeEach(() => {
		// Clear localStorage before each test
		localStorage.clear();
		vi.clearAllMocks();
	});

	describe('addToLocalStorage', () => {
		it('should add a new project to empty storage', () => {
			const setCountedLocalStorage = vi.fn();
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-1',
				userId: 123,
			};

			addToLocalStorage(undefined, projectToAdd, setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			expect(storedData).toBeDefined();
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(1);
			expect(parsed.projects[0]).toEqual(projectToAdd);
			expect(setCountedLocalStorage).toHaveBeenCalledWith(parsed);
		});

		it('should add a new project to existing storage', () => {
			const setCountedLocalStorage = vi.fn();
			const existingStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-2',
				userId: 456,
			};

			addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(2);
			expect(parsed.projects).toContainEqual({ projectId: 'project-1', userId: 123 });
			expect(parsed.projects).toContainEqual({ projectId: 'project-2', userId: 456 });
		});

		it('should not add duplicate project with same projectId and userId', () => {
			const setCountedLocalStorage = vi.fn();
			const existingStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-1',
				userId: 123,
			};

			addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);

			// Should not modify storage
			expect(setCountedLocalStorage).not.toHaveBeenCalled();
			expect(localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY)).toBeNull();
		});

		it('should not add project if existing project has userId and new project has null userId', () => {
			const setCountedLocalStorage = vi.fn();
			const existingStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-1',
				userId: null,
			};

			addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);

			// Should not modify storage
			expect(setCountedLocalStorage).not.toHaveBeenCalled();
			expect(localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY)).toBeNull();
		});

		it('should replace project with same projectId but different userId', () => {
			const setCountedLocalStorage = vi.fn();
			const existingStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-1',
				userId: 456,
			};

			addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(1);
			expect(parsed.projects[0]).toEqual({ projectId: 'project-1', userId: 456 });
		});

		it('should add project with null userId when no existing project has that projectId', () => {
			const setCountedLocalStorage = vi.fn();
			const existingStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-2',
				userId: null,
			};

			addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(2);
			expect(parsed.projects).toContainEqual({ projectId: 'project-1', userId: 123 });
			expect(parsed.projects).toContainEqual({ projectId: 'project-2', userId: null });
		});
	});

	describe('useAddToLocalStorage', () => {
		it('should call addToLocalStorage on mount', () => {
			const setCountedLocalStorage = vi.fn();
			const projectToAdd: CountedLocalStorageProject = {
				projectId: 'project-1',
				userId: 123,
			};

			act(() => {
				renderHook(() => useAddToLocalStorage(undefined, projectToAdd, setCountedLocalStorage));
			});

			expect(setCountedLocalStorage).toHaveBeenCalled();

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			expect(storedData).toBeDefined();
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toContainEqual(projectToAdd);
		});
	});

	describe('useInitializeLocalStorage', () => {
		it('should initialize from localStorage when data exists', () => {
			const existingData: CountedLocalStorage = {
				projects: [
					{ projectId: 'project-1', userId: 123 },
					{ projectId: 'project-2', userId: 456 },
				],
			};
			localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, JSON.stringify(existingData));

			const setCountedLocalStorage = vi.fn();

			act(() => {
				renderHook(() => useInitializeLocalStorage(setCountedLocalStorage));
			});

			expect(setCountedLocalStorage).toHaveBeenCalledWith(existingData);
		});

		it('should not call setter when localStorage is empty', () => {
			const setCountedLocalStorage = vi.fn();

			act(() => {
				renderHook(() => useInitializeLocalStorage(setCountedLocalStorage));
			});

			expect(setCountedLocalStorage).not.toHaveBeenCalled();
		});

		it('should handle invalid JSON in localStorage gracefully', () => {
			localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, 'invalid json');

			const setCountedLocalStorage = vi.fn();

			expect(() => {
				act(() => {
					renderHook(() => useInitializeLocalStorage(setCountedLocalStorage));
				});
			}).toThrow();
		});
	});

	describe('saveProjectEntry', () => {
		it('should skip if exact duplicate already in storage', async () => {
			const setCountedLocalStorage = vi.fn();
			const upsertFn = vi.fn();
			const storage: CountedLocalStorage = { projects: [{ projectId: 'p1', userId: 1 }] };

			await saveProjectEntry(true, { projectId: 'p1', userId: 1 }, storage, setCountedLocalStorage, upsertFn);

			expect(upsertFn).not.toHaveBeenCalled();
			expect(setCountedLocalStorage).not.toHaveBeenCalled();
		});

		it('should skip if trying to set userId=null when existing userId is set', async () => {
			const setCountedLocalStorage = vi.fn();
			const upsertFn = vi.fn();
			const storage: CountedLocalStorage = { projects: [{ projectId: 'p1', userId: 5 }] };

			await saveProjectEntry(true, { projectId: 'p1', userId: null }, storage, setCountedLocalStorage, upsertFn);

			expect(upsertFn).not.toHaveBeenCalled();
			expect(setCountedLocalStorage).not.toHaveBeenCalled();
		});

		it('auth path: calls upsertFn and updates state without writing to localStorage', async () => {
			const setCountedLocalStorage = vi.fn();
			const upsertFn = vi.fn().mockResolvedValue(undefined);
			const storage: CountedLocalStorage = { projects: [] };

			await saveProjectEntry(true, { projectId: 'p1', userId: 42 }, storage, setCountedLocalStorage, upsertFn);

			expect(upsertFn).toHaveBeenCalledWith('p1', 42);
			expect(setCountedLocalStorage).toHaveBeenCalled();
			expect(localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY)).toBeNull();
		});

		it('auth path: updates state correctly by replacing existing entry for same projectId', async () => {
			const setCountedLocalStorage = vi.fn();
			const upsertFn = vi.fn().mockResolvedValue(undefined);
			const storage: CountedLocalStorage = { projects: [{ projectId: 'p1', userId: null }] };

			await saveProjectEntry(true, { projectId: 'p1', userId: 7 }, storage, setCountedLocalStorage, upsertFn);

			expect(upsertFn).toHaveBeenCalledWith('p1', 7);
			const updater = setCountedLocalStorage.mock.calls[0][0];
			const result = updater({ projects: [{ projectId: 'p1', userId: null }] });
			expect(result.projects).toEqual([{ projectId: 'p1', userId: 7 }]);
		});

		it('anonymous path: delegates to addToLocalStorage (writes to localStorage)', async () => {
			const setCountedLocalStorage = vi.fn();
			const upsertFn = vi.fn();
			const storage: CountedLocalStorage = { projects: [] };

			await saveProjectEntry(false, { projectId: 'p1', userId: 3 }, storage, setCountedLocalStorage, upsertFn);

			expect(upsertFn).not.toHaveBeenCalled();
			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			expect(storedData).not.toBeNull();
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toContainEqual({ projectId: 'p1', userId: 3 });
		});
	});

	describe('removeFromLocalStorage', () => {
		it('should remove project from localStorage', () => {
			const setCountedLocalStorage = vi.fn();
			const countedLocalStorage: CountedLocalStorage = {
				projects: [
					{ projectId: 'project-1', userId: 123 },
					{ projectId: 'project-2', userId: 456 },
				],
			};

			removeFromLocalStorage(countedLocalStorage, 'project-1', setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(1);
			expect(parsed.projects[0]).toEqual({ projectId: 'project-2', userId: 456 });
			expect(setCountedLocalStorage).toHaveBeenCalledWith(parsed);
		});

		it('should handle removing non-existent project', () => {
			const setCountedLocalStorage = vi.fn();
			const countedLocalStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};

			removeFromLocalStorage(countedLocalStorage, 'non-existent', setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(1);
			expect(parsed.projects[0]).toEqual({ projectId: 'project-1', userId: 123 });
		});

		it('should handle undefined storage', () => {
			const setCountedLocalStorage = vi.fn();

			removeFromLocalStorage(undefined, 'project-1', setCountedLocalStorage);

			expect(setCountedLocalStorage).not.toHaveBeenCalled();
			expect(localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY)).toBeNull();
		});

		it('should remove all projects when removing the last one', () => {
			const setCountedLocalStorage = vi.fn();
			const countedLocalStorage: CountedLocalStorage = {
				projects: [{ projectId: 'project-1', userId: 123 }],
			};

			removeFromLocalStorage(countedLocalStorage, 'project-1', setCountedLocalStorage);

			const storedData = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
			const parsed: CountedLocalStorage = JSON.parse(storedData!);
			expect(parsed.projects).toHaveLength(0);
		});
	});

	describe('computeProjectsToSync', () => {
		it('returns projects in local but not on server', () => {
			const local: CountedLocalStorageProject[] = [
				{ projectId: 'p1', userId: 1 },
				{ projectId: 'p2', userId: 2 },
			];
			const server: CountedLocalStorageProject[] = [{ projectId: 'p2', userId: 2 }];
			expect(computeProjectsToSync(local, server)).toEqual([{ projectId: 'p1', userId: 1 }]);
		});

		it('returns empty array when all local projects are on server', () => {
			const local: CountedLocalStorageProject[] = [{ projectId: 'p1', userId: 1 }];
			const server: CountedLocalStorageProject[] = [{ projectId: 'p1', userId: 1 }];
			expect(computeProjectsToSync(local, server)).toEqual([]);
		});

		it('returns all local projects when server has none', () => {
			const local: CountedLocalStorageProject[] = [
				{ projectId: 'p1', userId: 1 },
				{ projectId: 'p2', userId: null },
			];
			expect(computeProjectsToSync(local, [])).toEqual(local);
		});

		it('returns empty array when local is empty', () => {
			const server: CountedLocalStorageProject[] = [{ projectId: 'p1', userId: 1 }];
			expect(computeProjectsToSync([], server)).toEqual([]);
		});

		it('preserves userId (including null) from local entries', () => {
			const local: CountedLocalStorageProject[] = [{ projectId: 'p1', userId: null }];
			expect(computeProjectsToSync(local, [])).toEqual([{ projectId: 'p1', userId: null }]);
		});
	});
});
