interface SummaryCardProps {
	myTotal: number;
	globalTotal: number;
}

function formatCurrency(value: number): string {
	return new Intl.NumberFormat('fr-FR', {
		style: 'currency',
		currency: 'EUR',
	}).format(value);
}

export const SummaryCard = (props: SummaryCardProps) => {
	return (
		<div className="">
			<div className="card-body p-4 space-y-3">
				<div className="flex justify-between items-center">
					<span className="text-base-content/70">Total global</span>

					<span className="font-semibold text-lg text-base-content/70">{formatCurrency(props.globalTotal)}</span>
				</div>
			</div>
		</div>
	);
};
