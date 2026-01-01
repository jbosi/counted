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
	return useMutation({
		mutationFn: (creatableUser: CreatableUser) => usersService.createUserAsync(creatableUser),
	});
}

export function useDeleteUser() {
	return useMutation({
		mutationFn: (userId: number) => usersService.deleteUserAsync(userId),
	});
}
