export const COUNTED_LOCAL_STORAGE_KEY = 'CountedLocalStorage';
export interface CountedLocalStorage {
	projects: [
		{
			projectId: string;
			userId: number;
		},
	];
}
