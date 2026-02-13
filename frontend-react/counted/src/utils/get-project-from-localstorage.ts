import type { CountedLocalStorage, CountedLocalStorageProject } from '../types/localStorage.model';

export const getProjectFromLocalstorage = (
	countedLocalStorage: CountedLocalStorage | undefined,
	projectId: string | undefined,
): CountedLocalStorageProject | undefined => {
	return countedLocalStorage?.projects.find((p) => p.projectId === projectId);
};

export const getProjectUserIdFromLocalstorage = (countedLocalStorage: CountedLocalStorage | undefined, projectId: string | undefined): number | undefined => {
	return getProjectFromLocalstorage(countedLocalStorage, projectId)?.userId ?? undefined;
};
