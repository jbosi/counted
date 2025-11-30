import { useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';

export function useUsersByProjectId(projectId: string) {
	return useQuery({
		queryKey: [`users-project-${projectId}`],
		queryFn: () => projectsService.getUsersByProjectIdAsync(projectId),
		refetchOnWindowFocus: false,
	});
}
