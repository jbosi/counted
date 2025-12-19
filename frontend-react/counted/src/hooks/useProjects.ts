import { useMutation, useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableProject } from '../types/projects.model';

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
	return useMutation({
		mutationFn: (creatableUser: CreatableProject) => projectsService.createProjectAsync(creatableUser),
	});
}

export function useDeleteProject() {
	return useMutation({
		mutationFn: (projectId: string) => projectsService.deleteProjectAsync(projectId),
	});
}
