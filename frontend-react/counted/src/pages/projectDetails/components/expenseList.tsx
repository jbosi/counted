import type { Expense } from '../../../types/expenses.model';
import { ExpenseItem } from './expenseItem';

export interface ExpenseListProps {
	expenses: Expense[];
}

export function ExpenseList(props: ExpenseListProps) {
	return (
		<ul className="flex flex-col gap-1">
			{props.expenses.map((e) => {
				return <ExpenseItem key={e.id} expense={e} />;
			})}
		</ul>
	);
}
