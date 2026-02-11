import { useContext } from 'react';
import type { User } from '../types/users.model';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';

interface SummaryCardProps {
	users: User[];
	globalTotal: number;
	projectId: string;
}

function formatCurrency(value: number): string {
	return new Intl.NumberFormat('fr-FR', {
		style: 'currency',
		currency: 'EUR',
	}).format(value);
}

export const SummaryCard = ({ users, globalTotal, projectId }: SummaryCardProps) => {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);

	const userId: number | undefined | null = countedLocalStorage?.projects.find((p) => p.projectId === projectId)?.userId;
	const me: User | undefined = users.find((u) => u.id === userId);

	return (
		<div>
			<div className="card-body p-4 space-y-3">
				<div className="flex justify-between items-center">
					<span className="text-base-content/70">Total des dépenses</span>
					<span className="font-semibold text-lg text-base-content/70">{formatCurrency(globalTotal ?? 0)}</span>
				</div>
				{me?.balance && (
					<div className="flex justify-between items-center">
						<span className="text-base-content/70">Mes dépenses</span>
						<span className="font-semibold text-lg text-base-content/70">{formatCurrency(me.balance)}</span>
					</div>
				)}
			</div>
		</div>
	);
};
