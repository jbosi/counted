import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useContext } from 'react';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';
import { projectsService } from '../services/projectsService';
import type { CreatableProject, EditableProject, ProjectDto, ProjectStatus } from '../types/projects.model';

export function useProjects(projectsIds: string[]) {
	return useQuery({
		queryKey: ['projects'],
		queryFn: () => projectsService.getByProjectIds(projectsIds.filter((p) => p != null)),
		refetchOnWindowFocus: false,
		enabled: projectsIds?.length > 0,
	});
}

export function useProject(projectId: string) {
	return useQuery({
		queryKey: [`project-${projectId}`],
		queryFn: () => projectsService.getByProjectId(projectId),
		refetchOnWindowFocus: false,
	});
}

export function useAddProject() {
	const { saveProjectEntry } = useContext(CountedLocalStorageContext);
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (creatableProject: CreatableProject) => projectsService.createProjectAsync(creatableProject),
		onSuccess: (data) => {
			queryClient.setQueryData(['projects'], (old: ProjectDto[]) => [...(old ?? []), data]);
			saveProjectEntry({ projectId: data.id, userId: null });
		},
	});
}

export function useEditProject() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (editableUser: EditableProject) => projectsService.editProjectAsync(editableUser),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'], exact: true });
		},
	});
}

export function useUpdateProjectStatus(projectId: string) {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (status: ProjectStatus) => projectsService.editProjectAsync({ id: projectId, status }),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'], exact: true });
			queryClient.invalidateQueries({ queryKey: [`project-${projectId}`] });
		},
	});
}

export function useDeleteProject() {
	const queryClient = useQueryClient();
	const { removeProjectEntry } = useContext(CountedLocalStorageContext);

	return useMutation({
		mutationFn: (projectId: string) => projectsService.deleteProjectAsync(projectId),
		onSuccess: (_, projectIdDeleted) => {
			queryClient.invalidateQueries({ queryKey: ['projects'], exact: true });
			removeProjectEntry(projectIdDeleted);
		},
	});
}
