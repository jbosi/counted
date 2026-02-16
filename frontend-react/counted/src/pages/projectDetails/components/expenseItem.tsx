import { useNavigate } from 'react-router';
import type { Expense } from '../../../types/expenses.model';
import { Avatar } from '../../../components/avatar';
import { getExpenseEmoji } from '../../../utils/get-expense-emojis';
import { useCallback } from 'react';

export interface ExpenseItemProps {
	expense: Expense;
}

export function ExpenseItem({ expense }: ExpenseItemProps) {
	const formattedAmount = expense.amount;
	const navigate = useNavigate();
	const emoji = useCallback(() => getExpenseEmoji(expense.name), [expense.name]);

	return (
		<li className="flex counted-listItems counted-listItems-hover shadow-sm" onClick={() => navigate(`./expenses/${expense.id}`)}>
			<Avatar name={emoji()} size="w-10" placeholderColor="transparent" />

			<div className="flex-1 min-w-0">
				<p className="text-base-content text-left truncate">{expense.name}</p>
			</div>

			<div className="text-right">
				<p className="text-sm text-base-content">{formattedAmount} €</p>
			</div>
		</li>
	);
}
