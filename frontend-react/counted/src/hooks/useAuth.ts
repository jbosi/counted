import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useContext } from 'react';
import { AuthContext } from '../contexts/authContext';
import { authService } from '../services/authService';
import type { LoginPayload, RegisterPayload } from '../types/auth.model';

export function useLogin() {
	const { setAccount } = useContext(AuthContext);
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (payload: LoginPayload) => authService.login(payload),
		onSuccess: (account) => {
			setAccount(account);
			queryClient.invalidateQueries({ queryKey: ['projects'] });
		},
	});
}

export function useRegister() {
	const { setAccount } = useContext(AuthContext);

	return useMutation({
		mutationFn: (payload: RegisterPayload) => authService.register(payload),
		onSuccess: (account) => {
			setAccount(account);
		},
	});
}

export function useLogout() {
	const { setAccount } = useContext(AuthContext);
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: () => authService.logout(),
		onSuccess: () => {
			setAccount(null);
			queryClient.clear();
		},
	});
}
