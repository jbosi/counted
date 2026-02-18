export interface DropdownActionProps {
	onDelete?: () => void;
	onEdit?: () => void;
}

export const DropdownAction = ({ onDelete, onEdit }: DropdownActionProps) => {
	return (
		<>
			<li>
				<button
					className="btn btn-primary btn-soft"
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
					className="btn btn-error btn-soft"
					onClick={(e) => {
						e.stopPropagation();
						onDelete?.();
					}}
				>
					Supprimer
				</button>
			</li>
		</>
	);
};
