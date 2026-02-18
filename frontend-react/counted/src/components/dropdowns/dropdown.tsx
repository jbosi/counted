import type { ReactNode } from 'react';

export interface DropdownProps {
	id: string;
	icon: ReactNode;
	children: ReactNode;
}

export const Dropdown = ({ id, children, icon }: DropdownProps) => {
	return (
		<div className="dropdown">
			<button
				role="button"
				className="btn btn-ghost btn-circle"
				popoverTarget={`popover-${id}`}
				style={{ anchorName: `--anchor-${id}` }}
				onClick={(e) => e.stopPropagation()}
			>
				{icon}
			</button>
			<ul className="dropdown menu w-52 rounded-box bg-base-100 shadow-sm" popover="auto" id={`popover-${id}`} style={{ positionAnchor: `--anchor-${id}` }}>
				{children}
			</ul>
		</div>
	);
};
