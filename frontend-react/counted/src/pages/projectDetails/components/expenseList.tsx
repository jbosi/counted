import { EmptyMagnifyingGlassIllustration } from '../../../shared/illustrations/emptyMagnifyingGlassIllustration';
import type { Expense } from '../../../types/expenses.model';
import { ExpenseItem } from './expenseItem';

export interface ExpenseListProps {
	expenses: Expense[];
}

function formatDate(dateStr: string): string {
	return new Date(dateStr).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' });
}

function groupExpensesByDate(expenses: Expense[]): [string, Expense[]][] {
	const map = new Map<string, Expense[]>();
	for (const expense of expenses) {
		const key = expense.date.split('T')[0];

		if (!map.has(key)) {
			map.set(key, []);
		}

		map.get(key)!.push(expense);
	}
	return [...map.entries()].sort(([a], [b]) => b.localeCompare(a));
}

export function ExpenseList(props: ExpenseListProps) {
	const groups = groupExpensesByDate(props.expenses);
	return (
		<div className="counted-list">
			{groups.length > 0 ? (
				groups.map(([date, expenses]) => (
					<div key={date}>
						<div className="divider divider-start text-sm font-medium">{formatDate(date)}</div>
						<ul className="counted-list">
							{expenses.map((e) => (
								<ExpenseItem key={e.id} expense={e} />
							))}
						</ul>
					</div>
				))
			) : (
				<div className="flex justify-center flex-col items-center">
					<span>Aucune dépense</span>
					<EmptyMagnifyingGlassIllustration />
				</div>
			)}
		</div>
	);
}
