import { useNavigate } from 'react-router';
import type { Expense } from '../types/expenses.model';
import { Avatar } from './avatar';

export interface ExpenseItemProps {
	expense: Expense;
}

export function ExpenseItem({ expense }: ExpenseItemProps) {
	const formattedAmount = expense.amount;
	const navigate = useNavigate();

	return (
		<div className="flex items-center gap-4 p-3 hover:bg-base-300 rounded-lg transition-colors" onClick={() => navigate(`./expenses/${expense.id}`)}>
			<Avatar name={'💵'} size={'w-10'} placeholderColor="var(--color-base-100)" />

			<div className="flex-1 min-w-0">
				<p className="font-semibold text-base-content truncate">{expense.name}</p>
			</div>

			<div className="text-right">
				<p className="font-bold text-lg text-base-content">{formattedAmount} €</p>
			</div>
		</div>
	);
}
