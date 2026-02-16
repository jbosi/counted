import type { Dispatch, SetStateAction } from 'react';
import { SettingsIcon } from '../../../shared/icons/settingsIcon';

interface ExpenseDropdownSettingsProps {
	showMyPaymentsState: [boolean, Dispatch<SetStateAction<boolean>>];
	showMyDebtsState: [boolean, Dispatch<SetStateAction<boolean>>];
}

export function ExpenseDropdownSettings({ showMyDebtsState, showMyPaymentsState }: ExpenseDropdownSettingsProps) {
	const [showMyPayments, setShowMyPayments] = showMyPaymentsState;
	const [showMyDebts, setShowMyDebts] = showMyDebtsState;

	return (
		<div className="dropdown dropdown-end self-end">
			<div className="dropdown dropdown-end self-end">
				<button
					role="button"
					className="btn btn-ghost btn-circle"
					popoverTarget="popover-settings"
					style={{ anchorName: '--anchor-settings' }}
					onClick={(e) => e.stopPropagation()}
				>
					<SettingsIcon />
				</button>
				<ul className="dropdown menu w-52 rounded-box bg-base-100 shadow-sm" popover="auto" id="popover-settings" style={{ positionAnchor: '--anchor-settings' }}>
					<li>
						<label className="label cursor-pointer justify-between gap-2">
							<span className="text-sm">Mes paiements</span>
							<input type="checkbox" className="toggle toggle-sm toggle-primary" checked={showMyPayments} onChange={(e) => setShowMyPayments(e.target.checked)} />
						</label>
					</li>
					<li>
						<label className="label cursor-pointer justify-between gap-2">
							<span className="text-sm">Mes dettes</span>
							<input type="checkbox" className="toggle toggle-sm toggle-primary" checked={showMyDebts} onChange={(e) => setShowMyDebts(e.target.checked)} />
						</label>
					</li>
				</ul>
			</div>
		</div>
	);
}
