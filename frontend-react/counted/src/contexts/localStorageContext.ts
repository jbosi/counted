import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { CountedLocalStorage } from '../types/localStorage.model';

export const CountedLocalStorageContext = createContext<CountedLocalStorageContextProperties>({ countedLocalStorage: undefined, setCountedLocalStorage: () => {} });

export interface CountedLocalStorageContextProperties {
	countedLocalStorage: CountedLocalStorage | undefined;
	setCountedLocalStorage: Dispatch<SetStateAction<CountedLocalStorage | undefined>>;
}
