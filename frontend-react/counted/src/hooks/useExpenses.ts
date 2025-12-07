import { useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';

export function useExpensesByProjectId(projectId: string) {
	return useQuery({
		queryKey: [`expenses-${projectId}`],
		queryFn: () => projectsService.getExpensesByProjectId(projectId),
		refetchOnWindowFocus: false,
	});
}
