import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableExpense } from '../types/expenses.model';
import { expensesService } from '../services/expensesService';

export function useExpensesByProjectId(projectId: string) {
	return useQuery({
		queryKey: [`expenses-${projectId}`],
		queryFn: () => projectsService.getExpensesByProjectId(projectId),
		refetchOnWindowFocus: false,
	});
}

export function useExpenseSummary(projectId: string) {
	return useQuery({
		queryKey: [`expenses-summary-${projectId}`],
		queryFn: () => projectsService.getExpensesSummaryByProjectId(projectId),
	});
}

export function useAddExpense() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (creatableUser: CreatableExpense) => expensesService.createExpenseAsync(creatableUser),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['expenses'] });
		},
	});
}
