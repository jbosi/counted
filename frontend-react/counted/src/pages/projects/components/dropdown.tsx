import type { ReactNode } from 'react';

export interface DropdownButtonProps {
	id: string;
	onDelete?: () => void;
	onEdit?: () => void;
	children: ReactNode;
}

export const DropdownButton = ({ id, onDelete, onEdit, children }: DropdownButtonProps) => {
	return (
		<div className="dropdown">
			<button
				role="button"
				className="btn btn-ghost btn-circle"
				popoverTarget={`popover-${id}`}
				style={{ anchorName: `--anchor-${id}` }}
				onClick={(e) => e.stopPropagation()}
			>
				{children}
			</button>
			<ul className="dropdown menu w-52 rounded-box bg-base-100 shadow-sm" popover="auto" id={`popover-${id}`} style={{ positionAnchor: `--anchor-${id}` }}>
				<li>
					<button
						className="btn btn-ghost"
						onClick={(e) => {
							e.stopPropagation();
							onEdit?.();
						}}
					>
						Editer
					</button>
				</li>
				<li>
					<button
						className="btn btn-ghost"
						onClick={(e) => {
							e.stopPropagation();
							onDelete?.();
						}}
					>
						Supprimer
					</button>
				</li>
			</ul>
		</div>
	);
};
