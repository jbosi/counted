import { zodResolver } from '@hookform/resolvers/zod';
import { useCallback, useContext, useState } from 'react';
import { useForm } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { addToLocalStorage } from '../../../hooks/useLocalStorage';
import { useAddProject } from '../../../hooks/useProjects';
import { useAddUsers } from '../../../hooks/useUsers';
import { PROJECT_FORM_SCHEMA } from './helpers/projectModal.helper';
import type { AddProjectModalProps, ProjectModalForm } from './models/projectModal.model';
import { ProjectModalContent } from './projectModalContent';
import { useNavigate } from 'react-router';

export function AddProjectModal({ dialogRef, modalId, closeDialogFn }: AddProjectModalProps) {
	const navigate = useNavigate();
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const [selectedUserName, setSelectedUserName] = useState<string | null>(null);
	const useFormReturn = useForm<ProjectModalForm>({
		resolver: zodResolver(PROJECT_FORM_SCHEMA),
		defaultValues: { users: [] },
	});

	const addProject = useAddProject();
	const addUsers = useAddUsers(addProject.data?.id ?? '');

	const setCurrentUserForProject = useCallback(
		(userId: number, projectId: string) => {
			addToLocalStorage(countedLocalStorage, { projectId, userId }, setCountedLocalStorage);
		},
		[countedLocalStorage, setCountedLocalStorage],
	);

	const onSubmit = async (data: ProjectModalForm): Promise<void> => {
		const createdProject = await addProject.mutateAsync({ name: data.projectName, description: data.projectDescription });
		const createdUsers = await addUsers.mutateAsync(data.users.map((u) => ({ name: u.name, projectId: createdProject?.id ?? '' })));

		const selectedUserId = createdUsers?.find((u) => u.name === selectedUserName)?.id;
		if (selectedUserId != null) {
			setCurrentUserForProject(selectedUserId, createdProject.id);
		}

		closeDialogFn();
		navigate(`projects/${createdProject.id}`);
	};

	return (
		<ProjectModalContent
			modalId={modalId}
			onSubmit={onSubmit}
			dialogRef={dialogRef}
			mutationHook={addProject}
			isSubmitLoading={addProject.isPending || addUsers.isPending}
			selectedUserName={selectedUserName}
			setSelectedUserName={setSelectedUserName}
			closeDialogFn={closeDialogFn}
			useFormReturn={useFormReturn}
		/>
	);
}
