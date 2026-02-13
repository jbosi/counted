import type { RefObject } from 'react';
import type { ProjectDto } from '../../../../types/projects.model';
import type { User } from '../../../../types/users.model';
import * as z from 'zod';
import type { PROJECT_FORM_SCHEMA } from '../helpers/projectModal.helper';

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

export type ProjectModalForm = z.infer<typeof PROJECT_FORM_SCHEMA>;
