import { type Dispatch, type SetStateAction, useEffect, useEffectEvent } from 'react';
import { type CountedLocalStorage, type CountedLocalStorageProject, COUNTED_LOCAL_STORAGE_KEY } from '../types/localStorage.model';

/**
 * Auth-aware upsert. When `isAuthenticated` is true, calls `upsertFn` and updates
 * in-memory state only (no localStorage write). When false, falls back to addToLocalStorage.
 * Applies the same idempotency guards as addToLocalStorage.
 */
export async function saveProjectEntry(
	isAuthenticated: boolean,
	entry: CountedLocalStorageProject,
	countedLocalStorage: CountedLocalStorage | undefined,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
	upsertFn: (projectId: string, userId: number | null) => Promise<void>,
): Promise<void> {
	const existing = countedLocalStorage?.projects.find((p) => p.projectId === entry.projectId);
	if (existing?.userId === entry.userId) return;
	if (existing?.userId != null && entry.userId == null) return;

	if (isAuthenticated) {
		await upsertFn(entry.projectId, entry.userId);
		setCountedLocalStorage((prev) => ({
			projects: [...(prev?.projects.filter((p) => p.projectId !== entry.projectId) ?? []), entry],
		}));
	} else {
		addToLocalStorage(countedLocalStorage, entry, setCountedLocalStorage);
	}
}

export function useAddToLocalStorage(
	existingStorage: CountedLocalStorage | undefined,
	projectToAdd: CountedLocalStorageProject,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	useEffect(() => {
		addToLocalStorage(existingStorage, projectToAdd, setCountedLocalStorage);
	}, []);
}

export function addToLocalStorage(
	existingStorage: CountedLocalStorage | undefined,
	projectToAdd: CountedLocalStorageProject,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	const isPreviouslyStored = existingStorage?.projects?.some((p) => p.projectId === projectToAdd.projectId && p.userId === projectToAdd.userId);
	if (isPreviouslyStored) {
		return;
	}

	const localStorageProject = existingStorage?.projects?.find((p) => p.projectId === projectToAdd.projectId);
	const isUserMissingInNewProject = localStorageProject?.userId != null && projectToAdd.userId === null;

	if (isUserMissingInNewProject) {
		return;
	}

	const newLocalStorage: CountedLocalStorage = removeProjectFromStorage(existingStorage, projectToAdd.projectId);

	newLocalStorage.projects.push({ projectId: projectToAdd.projectId, userId: projectToAdd.userId });

	localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, JSON.stringify(newLocalStorage));

	setCountedLocalStorage(newLocalStorage);
}

function removeProjectFromStorage(existingStorage: CountedLocalStorage | undefined, projectId: string): CountedLocalStorage {
	const isProjectIdAlreadyStored = existingStorage?.projects?.some((p) => p.projectId === projectId);

	if (!isProjectIdAlreadyStored) {
		return structuredClone(existingStorage ?? { projects: [] });
	}

	return structuredClone({ projects: existingStorage?.projects?.filter((p) => p.projectId !== projectId) ?? [] });
}

export function useInitializeLocalStorage(setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>) {
	const updateCountedLocalStorage = useEffectEvent(() => {
		const storage = localStorage.getItem(COUNTED_LOCAL_STORAGE_KEY);
		if (storage === null) {
			return;
		}
		setCountedLocalStorage(JSON.parse(storage));
	});
	useEffect(() => updateCountedLocalStorage(), []);
}

export function removeFromLocalStorage(
	countedLocalStorage: CountedLocalStorage | undefined,
	projectId: string,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	if (!countedLocalStorage) {
		return;
	}

	const filteredLocalStorage: CountedLocalStorageProject[] = countedLocalStorage?.projects.filter((p) => p.projectId !== projectId);
	const newLocalStorage: CountedLocalStorage = structuredClone({ projects: filteredLocalStorage });

	localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, JSON.stringify(newLocalStorage));
	setCountedLocalStorage(newLocalStorage);
}
