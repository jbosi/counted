import { Avatar } from '../../components/avatar';
import type { ReimbursementSuggestion } from '../../types/summary.model';
import type { User } from '../../types/users.model';

interface ReimbursementSuggestionsProps {
	reimbursementSuggestions: ReimbursementSuggestion[] | undefined;
	users: User[];
}

export function ReimbursementSuggestions({ reimbursementSuggestions, users }: ReimbursementSuggestionsProps) {
	if (reimbursementSuggestions === undefined) {
		return <></>;
	}

	return (
		<>
			<ul className="list bg-base-100 rounded-box shadow-md">
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

						return <ReimbursementSuggestionsItem amount={result.reimbursementSuggestions.amount} debtor={result.debtor} payer={result.payer} key={index} />;
					})}
			</ul>
		</>
	);
}

interface ReimbursementSuggestionsItemProps {
	debtor: User;
	payer: User;
	amount: number;
}

function ReimbursementSuggestionsItem({ debtor, payer, amount }: ReimbursementSuggestionsItemProps) {
	return (
		<>
			<li className="list-row reimbursement-list">
				<div className="flex flex-row gap-1.5 items-center">
					<div>
						<Avatar name={debtor.name} size={'w-8'} />
					</div>
					<div>
						{' '}
						<svg
							xmlns={'http://www.w3.org/2000/svg'}
							width={'14'}
							height={'24'}
							viewBox={'0 0 14 24'}
							fill={'none'}
							stroke={'currentColor'}
							strokeWidth={'1.5'}
							strokeLinecap={'round'}
							strokeLinejoin={'round'}
						>
							<polyline points="6 18 12 12 0 12 12 12 6 6" />
						</svg>
					</div>
					<div>
						<Avatar name={payer.name} size={'w-8'} />
					</div>
				</div>
				<div>
					<div>
						{debtor.name} doit à {payer.name}
					</div>
					<div className="text-xs uppercase font-semibold opacity-60">{amount} €</div>
				</div>
			</li>
		</>
	);
}
