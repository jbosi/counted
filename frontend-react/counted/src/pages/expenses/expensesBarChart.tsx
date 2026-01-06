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
	const width: number = (Math.abs(summaryAmount) * 30.0) / maxAmount;

	return (
		<div className="flex gap-2 justify-between">
			<div className="flex gap-2">
				<Avatar name={props.user.name} size="w-12" />
				<div className="self-center w-20 text-end">
					<span className={getTextClass(summaryAmount, isPositiveAmount)}>{`${isPositiveAmount ? '+' : ''}${summaryAmount} €`}</span>
				</div>
			</div>
			<progress
				className={`progress progress-summary self-center ${isPositiveAmount ? 'progress-primary' : 'progress-error'}`}
				style={{
					width: `${width}%`,
					transform: `translateX(-${isPositiveAmount ? 100 - width * 3 : 100}px)`,
				}}
				value={'100'}
				max={'100'}
			/>
		</div>
	);
}

function getTextClass(summaryAmount: number, isPositiveAmount: boolean): string {
	if (summaryAmount === 0) {
		return '';
	}

	return isPositiveAmount ? 'text-primary' : 'text-error';
}
