import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { User } from '../types/users.model';

export const ProjectUsersContext = createContext<ProjectUsersContextProperties>({ projectUsers: undefined, setProjectUsers: () => {} });

export interface ProjectUsersContextProperties {
	projectUsers: User[] | undefined;
	setProjectUsers: Dispatch<SetStateAction<User[] | undefined>>;
}
