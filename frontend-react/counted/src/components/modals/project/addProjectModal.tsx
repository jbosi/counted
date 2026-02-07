import { useCallback, useContext, useState } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { addToLocalStorage } from '../../../hooks/useLocalStorage';
import { useAddProject } from '../../../hooks/useProjects';
import { useAddUsers } from '../../../hooks/useUsers';
import type { CreatableUser, User } from '../../../types/users.model';
import { PROJECT_FORM_SCHEMA } from './helpers/projectModal.helper';
import type { AddProjectModalProps, ProjectModalForm } from './models/projectModal.model';
import { ProjectModalContent } from './projectModalContent';

export function AddProjectModal({ dialogRef, modalId, closeDialogFn }: AddProjectModalProps) {
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const [users, setUsers] = useState<(User | CreatableUser)[]>([]);
	const [selectedUserName, setSelectedUserName] = useState<string | null>(null);
	const [projectErrorState, setProjectErrorState] = useState<string | null>(null);
	const { register, formState, getValues } = useForm<ProjectModalForm>();

	const addProject = useAddProject();
	const addUsers = useAddUsers(addProject.data?.id ?? '');

	const setCurrentUserForProject = useCallback(
		(userId: number, projectId: string) => {
			addToLocalStorage(countedLocalStorage, { projectId, userId }, setCountedLocalStorage);
		},
		[countedLocalStorage, setCountedLocalStorage],
	);

	const onSubmit: SubmitHandler<ProjectModalForm> = async (data) => {
		const formValues: ProjectModalForm & { users: (User | CreatableUser)[] } = { ...data, users };
		const parsedResult = PROJECT_FORM_SCHEMA.safeParse(formValues);

		if (parsedResult.error) {
			setProjectErrorState(parsedResult.error.message);
			return;
		}

		const createdProject = await addProject.mutateAsync({ name: data.projectName, description: data.projectDescription });
		const createdUsers = await addUsers.mutateAsync(users.map((u) => ({ name: u.name, projectId: createdProject?.id ?? '' })));

		const selectedUserId = createdUsers?.find((u) => u.name === selectedUserName)?.id;
		if (selectedUserId != null) {
			setCurrentUserForProject(selectedUserId, createdProject.id);
		}

		closeDialogFn();
	};

	return (
		<ProjectModalContent
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
			selectedUserName={selectedUserName}
			setSelectedUserName={setSelectedUserName}
			closeDialogFn={closeDialogFn}
		/>
	);
}
