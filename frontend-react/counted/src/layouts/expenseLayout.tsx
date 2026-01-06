import { useEffect, useState } from 'react';
import { Outlet, useParams } from 'react-router';
import { Loading } from '../components/loading';
import { ExpenseContext } from '../contexts/expenseContext';
import { useExpense } from '../hooks/useExpenses';
import type { Expense } from '../types/expenses.model';

export function ExpenseLayout() {
	const [expense, setExpense] = useState<Expense | undefined>();
	const { expenseId } = useParams();

	const { data, isLoading } = useExpense(parseInt(expenseId ?? '')); // TODO : can i remove undefined ?

	useEffect(() => {
		setExpense(data);
	}, [data]);

	return <ExpenseContext value={{ expense, setExpense }}>{isLoading ? <Loading /> : <Outlet />}</ExpenseContext>;
}
