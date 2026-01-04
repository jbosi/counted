import { type Dispatch, type SetStateAction, useEffect, useEffectEvent } from 'react';
import { type CountedLocalStorage, COUNTED_LOCAL_STORAGE_KEY } from '../types/localStorage.model';

export function useAddLocalStorage(
	countedLocalStorage: CountedLocalStorage | undefined,
	projectId: string,
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>,
) {
	useEffect(() => {
		const isAlreadyStored = countedLocalStorage?.projects.some((p) => p.projectId === projectId);
		if (isAlreadyStored) {
			return;
		}
		const newLocalStorage: CountedLocalStorage = structuredClone(countedLocalStorage ?? { projects: [] }) as CountedLocalStorage;
		newLocalStorage.projects.push({ projectId: projectId, userId: 0 });
		localStorage.setItem(COUNTED_LOCAL_STORAGE_KEY, JSON.stringify(newLocalStorage));
		setCountedLocalStorage(newLocalStorage);
	}, [countedLocalStorage, projectId, setCountedLocalStorage]);
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
