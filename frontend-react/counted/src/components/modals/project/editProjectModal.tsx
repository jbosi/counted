import type { RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useEditProject } from '../../../hooks/useProjects';
import { ProjectModalContent } from './projectModalContent';
import type { ProjectDto } from '../../../types/projects.model';

export interface EditProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	project: ProjectDto;
}

export interface EditProjectModalForm {
	projectName: string;
	projectDescription: string;
}

export function EditProjectModal({ dialogRef, modalId, project }: EditProjectModalProps) {
	const { register, formState, getValues } = useForm<EditProjectModalForm>({
		defaultValues: {
			projectDescription: project.description,
			projectName: project.name,
		},
	});
	const hook = useEditProject();

	const onSubmit: SubmitHandler<EditProjectModalForm> = (data) => {
		hook.mutate({ name: data.projectName, description: data.projectDescription, id: project.id });
		dialogRef.current?.close();
	};

	return (
		<ProjectModalContent
			isEdition={true}
			modalId={modalId}
			onSubmit={onSubmit}
			dialogRef={dialogRef}
			formState={formState}
			getValues={getValues}
			mutationHook={hook}
			register={register}
		/>
	);
}
