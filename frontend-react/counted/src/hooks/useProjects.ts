import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableProject, EditableProject } from '../types/projects.model';

export function useProjects() {
	return useQuery({
		queryKey: ['projects'],
		queryFn: () => projectsService.getAllAsync(),
		refetchOnWindowFocus: false,
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
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (creatableProject: CreatableProject) => projectsService.createProjectAsync(creatableProject),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'], exact: true });
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

export function useDeleteProject() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (projectId: string) => projectsService.deleteProjectAsync(projectId),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['projects'], exact: true });
		},
	});
}
