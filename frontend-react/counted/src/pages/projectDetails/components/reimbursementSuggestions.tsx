import { Avatar } from '../../../components/avatar';
import { DollarIcon } from '../../../shared/icons/dollarIcon';
import { RightArrowIcon } from '../../../shared/icons/righArrowIcon';
import { CheckMarkIllustration } from '../../../shared/illustrations/checkMarkIllustration';
import type { ReimbursementSuggestion } from '../../../types/summary.model';
import type { User } from '../../../types/users.model';

interface ReimbursementSuggestionsProps {
	reimbursementSuggestions: ReimbursementSuggestion[] | undefined;
	users: User[];
	onReimburse?: (suggestion: ReimbursementSuggestion) => void;
}

export function ReimbursementSuggestions({ reimbursementSuggestions, users, onReimburse }: ReimbursementSuggestionsProps) {
	if (reimbursementSuggestions === undefined || reimbursementSuggestions?.length === 0) {
		return (
			<div className="flex justify-center flex-col items-center">
				<CheckMarkIllustration />
				<span className="font-bold">Les comptes sont bons !</span>
				<span>Des suggestions de remboursement seront proposées ici si les comptes ne sont pas équilibrés</span>
			</div>
		);
	}

	return (
		<ul className="counted-list">
			{reimbursementSuggestions
				.map((reimbursementSuggestions) => {
					const debtor = users?.find((u) => u.id === reimbursementSuggestions.userIdDebtor);
					const payer = users?.find((u) => u.id === reimbursementSuggestions.userIdPayer);

					return {
						debtor,
						payer,
						reimbursementSuggestions,
					};
				})
				.sort((a, b) => (a.debtor?.name ?? '').localeCompare(b.debtor?.name ?? ''))
				.map((result, index) => {
					if (!result.debtor || !result.payer) {
						return null;
					}

					return (
						<ReimbursementSuggestionsItem
							amount={result.reimbursementSuggestions.amount}
							debtor={result.debtor}
							payer={result.payer}
							suggestion={result.reimbursementSuggestions}
							onReimburse={onReimburse}
							key={index}
						/>
					);
				})}
		</ul>
	);
}

interface ReimbursementSuggestionsItemProps {
	debtor: User;
	payer: User;
	amount: number;
	suggestion: ReimbursementSuggestion;
	onReimburse?: (suggestion: ReimbursementSuggestion) => void;
}

function ReimbursementSuggestionsItem({ debtor, payer, amount, suggestion, onReimburse }: ReimbursementSuggestionsItemProps) {
	return (
		<li className="grid reimbursement-list counted-listItems shadow-sm">
			<div className="flex flex-row gap-1.5 items-center">
				<Avatar name={debtor.name} size="w-8" />
				<RightArrowIcon />
				<Avatar name={payer.name} size="w-8" />
			</div>
			<div>
				<div>
					{debtor.name} doit à {payer.name}
				</div>
				<div className="text-xs uppercase font-semibold opacity-60">{amount} €</div>
			</div>
			{onReimburse && (
				<button type="button" role="button" className="btn btn-circle" onClick={() => onReimburse(suggestion)}>
					<DollarIcon />
				</button>
			)}
		</li>
	);
}
