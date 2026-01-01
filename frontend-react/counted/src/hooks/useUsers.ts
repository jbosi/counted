import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableUser, User } from '../types/users.model';
import { usersService } from '../services/usersService';

export function useUsersByProjectId(projectId: string) {
	return useQuery({
		queryKey: ['users', 'project', projectId],
		queryFn: () => projectsService.getUsersByProjectIdAsync(projectId),
		refetchOnWindowFocus: false,
	});
}

export function useAddUser(projectId: string) {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (creatableUser: CreatableUser) => usersService.createUserAsync(creatableUser),
		onSuccess: (data) => {
			queryClient.setQueryData(['users', 'project', projectId], (old: User[]) => [...old, data]);
		},
	});
}

export function useDeleteUser(projectId: string) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (userId: number) => usersService.deleteUserAsync(userId),
		onSuccess: (_, userId) => {
			queryClient.setQueryData(['users', 'project', projectId], (old: User[]) => old.filter((o) => o.id !== userId));
		},
	});
}
