import { Avatar } from '../../components/avatar';
import type { User } from '../../types/users.model';
import './expensesBarChart.css';

export interface ExpenseBarChartProps {
	user: User;
	summaryAmount: number;
	maxAmount: number;
}

export function ExpenseBarChartComponent(props: ExpenseBarChartProps) {
	const summaryAmount: number = props.summaryAmount;
	const maxAmount: number = props.maxAmount;
	const isPositiveAmount: boolean = summaryAmount > 0;

	return (
		<div className="flex gap-2 justify-between">
			<div className="flex gap-2">
				<Avatar name={props.user.name} size="w-12" />
				<div className="self-center">
					<span className={`${isPositiveAmount ? 'text-primary' : 'text-error'}`}>{`${isPositiveAmount ? '+' : ''}${summaryAmount} €`}</span>
				</div>
			</div>
			<progress
				className={`progress self-center ${isPositiveAmount ? 'progress-primary' : 'progress-error'}`}
				style={{
					width: `${(Math.abs(summaryAmount) * 30.0) / maxAmount}%`,
				}}
				value={'100'}
				max={'100'}
			/>
		</div>
	);
}
