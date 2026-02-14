import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useState } from 'react';
import { useForm } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { addToLocalStorage } from '../../../hooks/useLocalStorage';
import { useEditProject } from '../../../hooks/useProjects';
import { useAddUsers, useDeleteUser } from '../../../hooks/useUsers';
import { PROJECT_FORM_SCHEMA } from './helpers/projectModal.helper';
import type { EditProjectModalProps, ProjectModalForm } from './models/projectModal.model';
import { ProjectModalContent } from './projectModalContent';

export function EditProjectModal({ dialogRef, modalId, project, users: initialUsers, closeDialogFn }: EditProjectModalProps) {
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const [selectedUserName, setSelectedUserName] = useState<string | null>(null);
	const useFormReturn = useForm<ProjectModalForm>({
		resolver: zodResolver(PROJECT_FORM_SCHEMA),
		defaultValues: {
			projectDescription: project.description,
			projectName: project.name,
			users: initialUsers.map((u) => ({ name: u.name, userId: u.id })),
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

	const onSubmit = async (data: ProjectModalForm): Promise<void> => {
		await editProject.mutateAsync({ name: data.projectName, description: data.projectDescription, id: project.id });

		const newUsers = data.users.filter((u) => u.userId == null);
		const createdUsers = await addUsers.mutateAsync(newUsers.map((u) => ({ name: u.name, projectId: project.id })));

		const currentUserIds = new Set(data.users.filter((u) => u.userId != null).map((u) => u.userId!));
		const userIdsToRemove = initialUsers.map((u) => u.id).filter((uId) => !currentUserIds.has(uId));
		for (const userIdToRemove of userIdsToRemove) {
			await removeUser.mutateAsync(userIdToRemove);
		}

		const selectedUserId = [...initialUsers, ...createdUsers]?.find((u) => u.name === selectedUserName)?.id;
		if (selectedUserId != null) {
			setCurrentUserForProject(selectedUserId, project.id);
		}

		closeDialogFn();
	};

	return (
		<ProjectModalContent
			projectId={project.id}
			modalId={modalId}
			onSubmit={onSubmit}
			dialogRef={dialogRef}
			mutationHook={editProject}
			isSubmitLoading={editProject.isPending || addUsers.isPending}
			selectedUserName={selectedUserName}
			setSelectedUserName={setSelectedUserName}
			closeDialogFn={closeDialogFn}
			useFormReturn={useFormReturn}
		/>
	);
}
