import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableUser, User } from '../types/users.model';
import { usersService } from '../services/usersService';
import { useContext } from 'react';
import { ProjectUsersContext } from '../contexts/projectUsersContext';

export function useUsersByProjectId(projectId: string | undefined) {
	return useQuery({
		queryKey: ['users', 'project', projectId],
		queryFn: () => projectsService.getUsersByProjectIdAsync(projectId),
		refetchOnWindowFocus: false,
		enabled: !!projectId,
	});
}

export function useAddUsers(projectId: string) {
	const queryClient = useQueryClient();
	const { projectUsers, setProjectUsers } = useContext(ProjectUsersContext);

	return useMutation({
		mutationFn: (creatableUsers: CreatableUser[]) => usersService.createUsersAsync(creatableUsers),
		onSuccess: (newUsers) => {
			queryClient.setQueryData(['users', 'project', projectId], (old: User[] | undefined) => [...(old ?? []), ...newUsers]);
			setProjectUsers([...(projectUsers ?? []), ...newUsers]);
		},
	});
}

export function useDeleteUser(projectId: string) {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (userId: number) => usersService.deleteUserAsync(userId),
		onSuccess: (_, userId) => {
			queryClient.setQueryData(['users', 'project', projectId], (old: User[] | undefined) => (old ?? []).filter((o) => o.id !== userId));
		},
	});
}
