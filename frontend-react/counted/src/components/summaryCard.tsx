import { useContext, useMemo } from 'react';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';
import { usePaymentsByProjectId } from '../hooks/usePayments';
import { getProjectUserIdFromLocalstorage } from '../utils/get-project-from-localstorage';
import { useExpensesByProjectId } from '../hooks/useExpenses';

interface SummaryCardProps {
	globalTotal: number;
	projectId: string;
}

function formatCurrency(value: number): string {
	return new Intl.NumberFormat('fr-FR', {
		style: 'currency',
		currency: 'EUR',
	}).format(value);
}

export const SummaryCard = ({ globalTotal, projectId }: SummaryCardProps) => {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const { data: payments } = usePaymentsByProjectId(projectId);
	const { data: expenses } = useExpensesByProjectId(projectId);
	const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

	// If calculated from the backend
	// const userId: number | undefined = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);
	// const me: User | undefined = users.find((u) => u.id === userId);

	const myPaymentsSum: number = useMemo(() => {
		if (!payments || storedUserId == null) {
			return 0;
		}

		const myPayments = payments?.filter((p) => p.userId === storedUserId);

		return myPayments?.filter((p) => expenses?.find((e) => e.id === p.expenseId)?.expenseType === 'Expense' && p.isDebt).reduce((acc, val) => acc + val.amount, 0);
	}, [expenses, payments, storedUserId]);

	return (
		<div>
			<div className="card-body pt-4 pb-4 pl-0 pr-0">
				<div className="flex justify-between items-center">
					<span className="text-base-content">Total des dépenses</span>
					<span className="font-semibold text-lg text-base-content">{formatCurrency(globalTotal ?? 0)}</span>
				</div>
				<div className="flex justify-between items-center">
					<span className="text-base-content">Mes dépenses</span>
					<span className="font-semibold text-lg text-base-content">{formatCurrency(myPaymentsSum)}</span>
				</div>
			</div>
		</div>
	);
};
