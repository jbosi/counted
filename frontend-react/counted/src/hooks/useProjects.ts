import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useContext } from 'react';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';
import { projectsService } from '../services/projectsService';
import type { CreatableProject, EditableProject, ProjectDto, ProjectStatus } from '../types/projects.model';

export function useProjects(projectsIds: string[]) {
	const sortedIds = projectsIds.slice().sort();
	return useQuery({
		queryKey: ['projects', ...sortedIds],
		queryFn: () => projectsService.getByProjectIds(sortedIds),
		refetchOnWindowFocus: false,
		enabled: sortedIds.length > 0,
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
			queryClient.setQueriesData({ queryKey: ['projects'] }, (old: ProjectDto[] | undefined) => [...(old ?? []), data]);
			saveProjectEntry({ projectId: data.id, userId: null });
		},
	});
}

export function useEditProject() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (editableUser: EditableProject) => projectsService.editProjectAsync(editableUser),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'] });
		},
	});
}

export function useUpdateProjectStatus(projectId: string) {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (status: ProjectStatus) => projectsService.editProjectAsync({ id: projectId, status }),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'] });
			queryClient.invalidateQueries({ queryKey: [`project-${projectId}`] });
		},
	});
}

export function useDeleteProject() {
	const { removeProjectEntry } = useContext(CountedLocalStorageContext);

	return useMutation({
		mutationFn: (projectId: string) => projectsService.deleteProjectAsync(projectId),
		onSuccess: (_, projectIdDeleted) => {
			removeProjectEntry(projectIdDeleted); // key change via countedLocalStorage drives the refetch
		},
	});
}
