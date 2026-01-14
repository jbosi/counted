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
	const width: number = (Math.abs(summaryAmount) * 100.0) / maxAmount;

	return (
		<div className="grid expenseBarChart-row">
			<Avatar name={props.user.name} size="w-12" />
			<div className="self-center w-20 text-center">
				<span className={getTextClass(summaryAmount, isPositiveAmount)}>{`${isPositiveAmount ? '+' : ''}${summaryAmount} €`}</span>
			</div>
			{isPositiveAmount ? <span></span> : <></>}
			<progress
				className={`progress progress-summary self-center ${isPositiveAmount ? 'progress-primary' : 'progress-error justify-self-end'}`}
				style={{
					width: `${width}%`,
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
