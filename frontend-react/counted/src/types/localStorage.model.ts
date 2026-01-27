export const COUNTED_LOCAL_STORAGE_KEY = 'CountedLocalStorage';
export interface CountedLocalStorage {
	projects: CountedLocalStorageProject[];
}

export interface CountedLocalStorageProject {
	projectId: string;
	userId: number | null;
}
