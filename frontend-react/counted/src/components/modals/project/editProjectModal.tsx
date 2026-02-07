import { useCallback, useContext, useState } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useEditProject } from '../../../hooks/useProjects';
import { useAddUsers, useDeleteUser } from '../../../hooks/useUsers';
import type { CreatableUser, User } from '../../../types/users.model';
import { PROJECT_FORM_SCHEMA } from './helpers/projectModal.helper';
import { ProjectModalContent } from './projectModalContent';
import type { EditProjectModalProps, ProjectModalForm } from './models/projectModal.model';
import { addToLocalStorage } from '../../../hooks/useLocalStorage';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';

export function EditProjectModal({ dialogRef, modalId, project, users: initialUsers }: EditProjectModalProps) {
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const [users, setUsers] = useState<(CreatableUser | User)[]>(initialUsers);
	const [selectedUserName, setSelectedUserName] = useState<string | null>(null);
	const [projectErrorState, setProjectErrorState] = useState<string | null>(null);
	const { register, formState, getValues } = useForm<ProjectModalForm>({
		defaultValues: {
			projectDescription: project.description,
			projectName: project.name,
		},
	});

	const setCurrentUserForProject = useCallback(
		(userId: number, projectId: string) => {
			addToLocalStorage(countedLocalStorage, { projectId, userId }, setCountedLocalStorage);
		},
		[countedLocalStorage, setCountedLocalStorage],
	);

	const editProject = useEditProject();
	const addUsers = useAddUsers(project.id);
	const removeUser = useDeleteUser(project.id);

	const onSubmit: SubmitHandler<ProjectModalForm> = async (data) => {
		const formValues: ProjectModalForm & { users: (User | CreatableUser)[] } = { ...data, users };
		const parsedResult = PROJECT_FORM_SCHEMA.safeParse(formValues);

		if (parsedResult.error) {
			setProjectErrorState(parsedResult.error.message);
			return;
		}

		await editProject.mutateAsync({ name: data.projectName, description: data.projectDescription, id: project.id });

		const createdUsers = await addUsers.mutateAsync(users.filter((u) => !('id' in u)).map((u) => ({ name: u.name, projectId: project.id })));
		const userIdsToRemove = initialUsers.map((u) => u.id).filter((uId) => !(users as User[]).some((u) => u.id === uId));
		for (const userIdToRemove of userIdsToRemove) {
			await removeUser.mutateAsync(userIdToRemove);
		}

		const selectedUserId = [...initialUsers, ...createdUsers]?.find((u) => u.name === selectedUserName)?.id;
		if (selectedUserId != null) {
			setCurrentUserForProject(selectedUserId, project.id);
		}

		dialogRef.current?.close();
	};

	return (
		<ProjectModalContent
			projectId={project.id}
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
			selectedUserName={selectedUserName}
			setSelectedUserName={setSelectedUserName}
		/>
	);
}
