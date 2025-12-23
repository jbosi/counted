import type { Expense } from '../types/expenses.model';
import { Avatar } from './avatar';

export interface ExpenseItemProps {
	expense: Expense;
}

export function ExpenseItem(props: ExpenseItemProps) {
	const expense: Expense = props.expense;
	const formatted_amount = expense.amount;

	return (
		<div className="flex items-center gap-4 p-3 hover:bg-base-300 rounded-lg transition-colors">
			<Avatar name={'💰'} size={'w-10'} />

			<div className="flex-1 min-w-0">
				<p className="font-semibold text-base-content truncate">{expense.name}</p>
			</div>

			<div className="text-right">
				<p className="font-bold text-lg text-base-content">{formatted_amount} €</p>
			</div>
		</div>
	);
}
