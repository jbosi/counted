import type { RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddProject } from '../../../hooks/useProjects';
import { ProjectModalContent } from './projectModalContent';

export interface AddProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
}

export interface AddProjectModalForm {
	projectName: string;
	projectDescription: string;
}

export function AddProjectModal({ dialogRef, modalId }: AddProjectModalProps) {
	const { register, formState, getValues } = useForm<AddProjectModalForm>();
	const hook = useAddProject();

	const onSubmit: SubmitHandler<AddProjectModalForm> = (data) => {
		hook.mutate({ name: data.projectName, description: data.projectDescription });
		dialogRef.current?.close();
	};

	return (
		<ProjectModalContent
			isEdition={false}
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
