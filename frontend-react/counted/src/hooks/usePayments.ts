import { useQuery } from '@tanstack/react-query';
import { expensesService } from '../services/expensesService';
import { projectsService } from '../services/projectsService';

export function usePaymentsByExpenseId(expenseId: number) {
	return useQuery({
		queryKey: ['payments', expenseId],
		queryFn: () => expensesService.getPaymentsByExpenseId(expenseId),
		refetchOnWindowFocus: false,
	});
}

export function usePaymentsByProjectId(projectId: string) {
	return useQuery({
		queryKey: ['payments', 'project', projectId],
		queryFn: () => projectsService.getPaymentsByProjectId(projectId),
		refetchOnWindowFocus: false,
	});
}
