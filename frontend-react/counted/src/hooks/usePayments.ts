import { useQuery } from '@tanstack/react-query';
import { expensesService } from '../services/expensesService';

export function usePaymentsByExpenseId(expenseId: number) {
	return useQuery({
		queryKey: ['payments', expenseId],
		queryFn: () => expensesService.getPaymentsByExpenseId(expenseId),
		refetchOnWindowFocus: false,
	});
}
