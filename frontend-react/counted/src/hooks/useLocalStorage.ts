import { type Dispatch, type SetStateAction, useEffect, useEffectEvent } from 'react';
import { type CountedLocalStorage, type CountedLocalStorageProject, COUNTED_LOCAL_STORAGE_KEY } from '../types/localStorage.model';

export function useAddToLocalStorage(
	countedLocalStorage: CountedLocalStorage | undefined,
	projectId: string,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	useEffect(() => {
		addToLocalStorage(countedLocalStorage, projectId, setCountedLocalStorage);
	}, [countedLocalStorage, projectId, setCountedLocalStorage]);
}

export function addToLocalStorage(
	countedLocalStorage: CountedLocalStorage | undefined,
	projectId: string,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	const isAlreadyStored = countedLocalStorage?.projects?.some((p) => p.projectId === projectId);
	if (isAlreadyStored) {
		return;
	}

	const newLocalStorage: CountedLocalStorage = structuredClone(countedLocalStorage ?? { projects: [] }) as CountedLocalStorage;
	newLocalStorage.projects.push({ projectId: projectId, userId: 0 });

	localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, JSON.stringify(newLocalStorage));
	setCountedLocalStorage(newLocalStorage);
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
