/* eslint-disable @typescript-eslint/no-unused-vars */
import type { ProjectSummary } from '../../../types/summary.model';
import type { User } from '../../../types/users.model';
import { ExpenseBarChartComponent } from '../../expenses/expensesBarChart';

interface ProjectSummaryProps {
	projectSummary: ProjectSummary | undefined;
	users: User[];
}

export function ProjectSummary({ projectSummary, users }: ProjectSummaryProps) {
	if (projectSummary === undefined) {
		return <></>;
	}

	const summary = projectSummary.summary;

	const usersWithoutExpense: User[] = users.filter((u) => !Object.entries(summary)?.some((s) => u.id === Number(s?.[0])));

	const maxAmount = Math.max(...Object.values(summary).map((v) => Math.abs(v)), 1);

	return (
		<ul className="counted-list">
			{Object.entries(summary)
				.sort(([_, amount1], [__, amount2]) => amount1 - amount2)
				.map(([userIdStr, amount]) => {
					const userId = Number(userIdStr);
					const user = users?.find((u) => u.id === userId);
					if (!user) {
						return null;
					}

					return <ExpenseBarChartComponent key={userId} user={user} summaryAmount={amount} maxAmount={maxAmount} />;
				})}
			{usersWithoutExpense.length > 0 &&
				usersWithoutExpense.map((user) => <ExpenseBarChartComponent key={user.id} user={user} summaryAmount={0} maxAmount={maxAmount} />)}
		</ul>
	);
}
