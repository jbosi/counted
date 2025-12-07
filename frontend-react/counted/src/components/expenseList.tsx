import type { Expense } from '../types/expenses.model';
import { ExpenseItem } from './expenseItem';

export interface ExpenseListProps {
	expenses: Expense[];
}

export function ExpenseList(props: ExpenseListProps) {
	return (
		<div>
			{props.expenses.map((e) => {
				return <ExpenseItem expense={e} />;
			})}
		</div>
	);
}
