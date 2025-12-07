import { useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';

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
