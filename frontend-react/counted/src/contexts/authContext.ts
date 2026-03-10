import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { Account } from '../types/auth.model';

export const AuthContext = createContext<AuthContextProps>({
	account: undefined,
	setAccount: () => {},
});

export interface AuthContextProps {
	// undefined = loading, null = unauthenticated, Account = authenticated
	account: Account | null | undefined;
	setAccount: Dispatch<SetStateAction<Account | null | undefined>>;
}
