import { useLoaderData } from 'react-router';
import { Avatar } from '../../components/avatar';
import { usePaymentsByExpenseId } from '../../hooks/usePayments';
import { useContext } from 'react';
import { ProjectUsersContext } from '../../contexts/projectUsersContext';
import type { PaymentViewModel } from '../../types/payments.model';

interface PaymentListProps {
	expenseId: number;
}

export function PaymentList() {
	const { expenseId }: PaymentListProps = useLoaderData();
	const { projectUsers } = useContext(ProjectUsersContext);

	const { data: payments, error, isError, isLoading } = usePaymentsByExpenseId(expenseId);
	const paymentsViewModel: PaymentViewModel[] = (payments ?? []).map((p) => ({
		amount: p.amount,
		createdAt: p.createdAt,
		expenseId: p.expenseId,
		id: p.id,
		isDebt: p.isDebt,
		user: projectUsers?.find((pu) => pu.id === p.userId),
	}));

	return (
		<div className="container p-4 max-w-md rounded-xl flex flex-col">
			{paymentsViewModel?.map((payment) => (
				<PaymentItem payment={payment} key={payment.id} />
			))}
		</div>
	);
}

interface PaymentItemProps {
	payment: PaymentViewModel;
}

function PaymentItem({ payment }: PaymentItemProps) {
	return (
		<div className="flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors">
			{/* Name */}
			<div className="flex-1 min-w-0 flex-row flex items-center gap-3">
				<Avatar name={payment.user?.name ?? '?'} />
				<p className="font-semibold text-base-content truncate">
					{payment.user?.name ?? '?'}
					{payment.isDebt ? ' doit' : ' a payé'}
				</p>
			</div>
			{/* Amount */}
			<div className="text-right">
				<p className="font-bold text-lg text-base-content">{payment.amount} €</p>
			</div>
		</div>
	);
}
