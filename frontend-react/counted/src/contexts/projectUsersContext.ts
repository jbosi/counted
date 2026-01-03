import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { User } from '../types/users.model';

export const ProjectUsersContext = createContext<ProjectUsersContextProperties>({ projectUsers: null, setProjectUsers: () => {} });

export interface ProjectUsersContextProperties {
	projectUsers: User[] | null | undefined;
	setProjectUsers: Dispatch<SetStateAction<User[] | null | undefined>>;
}
