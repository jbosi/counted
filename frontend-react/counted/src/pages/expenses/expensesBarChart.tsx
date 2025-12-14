import { Avatar } from '../../components/avatar';
import type { User } from '../../types/users.model';

export interface ExpenseBarChartProps {
	user: User;
	summaryAmount: number;
	maxAmount: number;
}

export function ExpenseBarChartComponent(props: ExpenseBarChartProps) {
	const summaryAmount: number = props.summaryAmount;
	const maxAmount: number = props.maxAmount;

	return (
		<div className="flex gap-2 justify-between">
			<div className="flex gap-2">
				<Avatar name={props.user.name} size="w-12" />
				<div className="self-center">
					<span>{`${summaryAmount > 0 ? '+' : '-'}${summaryAmount} €`}</span>
				</div>
			</div>
			<progress
				className={summaryAmount > 0 ? 'progress progress-primary self-center' : 'progress progress-error self-center'}
				style={
					summaryAmount > 0
						? {
								width: `${(Math.abs(summaryAmount) * 30.0) / maxAmount}%`,
							}
						: {
								transform: 'translateX(-100%);',
								width: `${(Math.abs(summaryAmount) * 30.0) / maxAmount}%`,
							}
				}
				value={'100'}
				max={'100'}
			/>
		</div>
	);
}
