import { useState, type RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import * as z from 'zod';
import { useEditProject } from '../../../hooks/useProjects';
import { useAddUsers, useDeleteUser } from '../../../hooks/useUsers';
import type { ProjectDto } from '../../../types/projects.model';
import type { CreatableUser, User } from '../../../types/users.model';
import { ProjectModalContent, type ProjectModalForm } from './projectModalContent';

export interface EditProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	project: ProjectDto;
	users: User[];
}

const formSchema = z.object({
	projectName: z.string().min(2).max(100),
	name: z.array(z.string().min(2).max(100)).min(1),
});

export function EditProjectModal({ dialogRef, modalId, project, users: initialUsers }: EditProjectModalProps) {
	const [users, setUsers] = useState<(CreatableUser | User)[]>(initialUsers);
	const [projectErrorState, setProjectErrorState] = useState<string | null>(null);
	const { register, formState, getValues } = useForm<ProjectModalForm>({
		defaultValues: {
			projectDescription: project.description,
			projectName: project.name,
		},
	});

	const editProject = useEditProject();
	const addUsers = useAddUsers(project.id);
	const removeUser = useDeleteUser(project.id);

	const onSubmit: SubmitHandler<ProjectModalForm> = async (data) => {
		const formValues: ProjectModalForm & { users: (User | CreatableUser)[] } = { ...data, users };
		const parsedResult = formSchema.safeParse(formValues);

		if (parsedResult.error) {
			setProjectErrorState(parsedResult.error.message);
			return;
		}

		await editProject.mutateAsync({ name: data.projectName, description: data.projectDescription, id: project.id });

		await addUsers.mutateAsync(users.filter((u) => !('id' in u)).map((u) => ({ name: u.name, projectId: project.id })));
		const userIdsToRemove = initialUsers.map((u) => u.id).filter((uId) => !(users as User[]).some((u) => u.id === uId));
		for (const userIdToRemove of userIdsToRemove) {
			await removeUser.mutateAsync(userIdToRemove);
		}

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
			mutationHook={editProject}
			register={register}
			users={users}
			setUsers={setUsers}
			projectErrorState={projectErrorState}
			isSubmitLoading={editProject.isPending || addUsers.isPending}
		/>
	);
}
