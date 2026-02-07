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
	const isZeroAmount: boolean = summaryAmount === 0;
	const width: number = Math.max((Math.abs(summaryAmount) * 100.0) / maxAmount, 2);

	return (
		<div className="grid expenseBarChart-row">
			<Avatar name={props.user.name} size="w-8" />
			<span className="self-center text-sm">{props.user.name}</span>
			<div className="self-center w-20 text-left ml-1">
				<span className={getTextClass(summaryAmount, isPositiveAmount)}>{`${isPositiveAmount ? '+' : ''}${summaryAmount} €`}</span>
			</div>
			<div className="flex">
				{isPositiveAmount ? <span className="w-full"></span> : <></>}
				<span className={`w-full ${isPositiveAmount ? 'text-left' : 'text-right'}`}>
					<progress
						className={`progress progress-summary self-center ${isPositiveAmount ? 'progress-primary' : isZeroAmount ? 'justify-self-end' : 'progress-error justify-self-end'}`}
						style={{
							width: `${width}%`,
						}}
						value={'100'}
						max={'100'}
					/>
				</span>
				{isPositiveAmount ? <></> : <span className="w-full"></span>}
			</div>
		</div>
	);
}

function getTextClass(summaryAmount: number, isPositiveAmount: boolean): string {
	if (summaryAmount === 0) {
		return '';
	}

	return isPositiveAmount ? 'text-primary' : 'text-error';
}
