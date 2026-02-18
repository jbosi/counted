import type { ReactNode } from 'react';
import { Dropdown } from './dropdown';

export interface DropdownActionProps {
	id: string;
	onDelete?: () => void;
	onEdit?: () => void;
	icon: ReactNode;
}

export const DropdownAction = ({ id, onDelete, onEdit, icon }: DropdownActionProps) => {
	return (
		<Dropdown icon={icon} id={id}>
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
		</Dropdown>
	);
};
