import { useState, type RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddProject } from '../../../hooks/useProjects';
import { ProjectModalContent, type ProjectModalForm } from './projectModalContent';
import type { CreatableUser, User } from '../../../types/users.model';
import * as z from 'zod';
import { useAddUsers } from '../../../hooks/useUsers';

export interface AddProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
}

const formSchema = z.object({
	projectName: z.string().min(2).max(100),
	users: z
		.array(
			z.object({
				name: z.string().min(2).max(100),
			}),
		)
		.min(1),
});

export function AddProjectModal({ dialogRef, modalId }: AddProjectModalProps) {
	const [users, setUsers] = useState<(User | CreatableUser)[]>([]);
	const [projectErrorState, setProjectErrorState] = useState<string | null>(null);
	const { register, formState, getValues } = useForm<ProjectModalForm>();

	const addProject = useAddProject();
	const addUsers = useAddUsers(addProject.data?.id ?? '');

	const onSubmit: SubmitHandler<ProjectModalForm> = async (data) => {
		const formValues: ProjectModalForm & { users: (User | CreatableUser)[] } = { ...data, users };
		const parsedResult = formSchema.safeParse(formValues);

		if (parsedResult.error) {
			setProjectErrorState(parsedResult.error.message);
			return;
		}

		const createdProject = await addProject.mutateAsync({ name: data.projectName, description: data.projectDescription });
		await addUsers.mutateAsync(users.map((u) => ({ name: u.name, projectId: createdProject?.id ?? '' })));
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
			mutationHook={addProject}
			register={register}
			users={users}
			setUsers={setUsers}
			projectErrorState={projectErrorState}
			isSubmitLoading={addProject.isPending || addUsers.isPending}
		/>
	);
}
