import { useMutation, useQuery } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableUser } from '../types/users.model';
import { usersService } from '../services/usersService';

export function useUsersByProjectId(projectId: string) {
	return useQuery({
		queryKey: [`users-project-${projectId}`],
		queryFn: () => projectsService.getUsersByProjectIdAsync(projectId),
		refetchOnWindowFocus: false,
	});
}

export function useAddUser() {
	// const queryClient = useQueryClient()

	return useMutation({
		// mutationKey: [`users-project-${creatableUser.projectId}`],
		// onSuccess: (creatableUser: CreatableUser) =>
		mutationFn: (creatableUser: CreatableUser) => usersService.cretateUserAsync({ name: creatableUser.name, project_id: creatableUser.projectId }),
	});
}
