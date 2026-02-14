import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { projectsService } from '../services/projectsService';
import type { CreatableExpense, EditableExpense, Expense } from '../types/expenses.model';
import { expensesService } from '../services/expensesService';

export function useExpensesByProjectId(projectId: string) {
	return useQuery({
		// queryKey: [`expenses-${projectId}`],
		queryKey: ['expenses'],
		queryFn: () => projectsService.getExpensesByProjectId(projectId),
		refetchOnWindowFocus: false,
	});
}

export function useExpense(expenseId: number) {
	return useQuery({
		queryKey: ['expense', expenseId],
		queryFn: () => expensesService.getExpenseById(expenseId),
		refetchOnWindowFocus: false,
	});
}

export function useExpenseSummary(projectId: string) {
	return useQuery({
		queryKey: ['expenses', 'summary', projectId],
		queryFn: () => projectsService.getExpensesSummaryByProjectId(projectId),
	});
}

export function useAddExpense() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (creatableExpense: CreatableExpense) => expensesService.createExpenseAsync(creatableExpense),
		onSuccess: (data) => {
			queryClient.setQueryData(['expenses'], (previous: Expense[] | undefined) => [...(previous ?? []), data]);
			queryClient.invalidateQueries({ queryKey: ['expenses', 'summary', data.projectId] });
			queryClient.invalidateQueries({ queryKey: ['payments', 'project'] });
		},
	});
}

export function useEditExpense() {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (editableExpense: EditableExpense) => expensesService.editExpenseAsync(editableExpense),
		onSuccess: (editedExpense) => {
			queryClient.setQueryData(['expenses'], (previous: Expense[] | undefined) => [
				...(previous?.filter((p) => p.projectId !== editedExpense.projectId) ?? []),
				editedExpense,
			]);
			queryClient.setQueryData(['expense', editedExpense.id], () => editedExpense);
			queryClient.invalidateQueries({ queryKey: ['payments', editedExpense.id] });
			queryClient.invalidateQueries({ queryKey: ['expenses', 'summary', editedExpense.projectId] });
			queryClient.invalidateQueries({ queryKey: ['payments', 'project'] });
		},
	});
}

export function useDeleteExpense(projectId: string) {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (expenseId: number) => expensesService.deleteExpense(expenseId),
		onSuccess: (_, expenseId) => {
			queryClient.setQueryData(['expenses'], (previous: Expense[] | undefined) => (previous ?? []).filter((o) => o.id !== expenseId));
			queryClient.invalidateQueries({ queryKey: ['expenses', 'summary', projectId] });
			queryClient.invalidateQueries({ queryKey: ['payments', 'project'] });
		},
	});
}
