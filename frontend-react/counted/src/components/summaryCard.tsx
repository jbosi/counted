import { useContext, useMemo } from 'react';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';
import { usePaymentsByProjectId } from '../hooks/usePayments';
import { getProjectUserIdFromLocalstorage } from '../utils/get-project-from-localstorage';

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
	const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

	// If calculated from the backend
	// const userId: number | undefined = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);
	// const me: User | undefined = users.find((u) => u.id === userId);

	const myPaymentsSum: number = useMemo(() => {
		if (!payments || storedUserId == null) {
			return 0;
		}
		return payments
			?.filter((p) => p.userId === storedUserId)
			.map((val) => val.amount)
			.reduce((acc, val) => acc + val, 0);
	}, [payments, storedUserId]);

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
