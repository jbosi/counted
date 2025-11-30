import { useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';

export function useProjects() {
	return useQuery({
		queryKey: ['projects'],
		queryFn: () => projectsService.getAllAsync(),
		refetchOnWindowFocus: false,
	});
}
