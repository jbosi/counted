import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useState } from 'react';
import { useForm } from 'react-hook-form';
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
	const useFormReturn = useForm<ProjectModalForm>({ resolver: zodResolver(PROJECT_FORM_SCHEMA) });

	const addProject = useAddProject();
	const addUsers = useAddUsers(addProject.data?.id ?? '');

	const setCurrentUserForProject = useCallback(
		(userId: number, projectId: string) => {
			addToLocalStorage(countedLocalStorage, { projectId, userId }, setCountedLocalStorage);
		},
		[countedLocalStorage, setCountedLocalStorage],
	);

	const onSubmit = async (data: ProjectModalForm): Promise<void> => {
		const formValues: ProjectModalForm = { ...data, users };

		const createdProject = await addProject.mutateAsync({ name: formValues.projectName, description: formValues.projectDescription });
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
			mutationHook={addProject}
			users={users}
			setUsers={setUsers}
			isSubmitLoading={addProject.isPending || addUsers.isPending}
			selectedUserName={selectedUserName}
			setSelectedUserName={setSelectedUserName}
			closeDialogFn={closeDialogFn}
			useFormReturn={useFormReturn}
		/>
	);
}
