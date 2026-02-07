import type { RefObject } from 'react';
import type { ProjectDto } from '../../../../types/projects.model';
import type { User } from '../../../../types/users.model';

export interface AddProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	closeDialogFn: () => void;
}

export interface EditProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	project: ProjectDto;
	users: User[];
	closeDialogFn: () => void;
}

export interface ProjectModalForm {
	projectName: string;
	projectDescription: string;
}
