import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { CountedLocalStorage, CountedLocalStorageProject } from '../types/localStorage.model';

export const CountedLocalStorageContext = createContext<CountedLocalStorageContextProperties>({
	countedLocalStorage: undefined,
	setCountedLocalStorage: () => {},
	saveProjectEntry: async () => {},
	removeProjectEntry: async () => {},
});

export interface CountedLocalStorageContextProperties {
	countedLocalStorage: CountedLocalStorage | undefined;
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>;
	saveProjectEntry: (entry: CountedLocalStorageProject) => Promise<void>;
	removeProjectEntry: (projectId: string) => Promise<void>;
}
