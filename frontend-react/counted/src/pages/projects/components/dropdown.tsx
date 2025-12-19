interface DropdownButtonProps {
	id: string;
	onDelete?: () => void;
	onEdit?: () => void;
}

export const DropdownButton = (props: DropdownButtonProps) => {
	return (
		<div className="dropdown">
			<button
				role="button"
				className="btn btn-ghost btn-circle"
				popoverTarget={`popover-${props.id}`}
				style={{ anchorName: `--anchor-${props.id}` }}
				onClick={(e) => e.stopPropagation()}
			>
				...
			</button>
			<ul className="dropdown menu w-52 rounded-box bg-base-100 shadow-sm" popover="auto" id={`popover-${props.id}`} style={{ positionAnchor: `--anchor-${props.id}` }}>
				<li>
					<button
						className="btn btn-ghost"
						onClick={(e) => {
							e.stopPropagation();
							props?.onEdit?.();
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
							props?.onDelete?.();
						}}
					>
						Supprimer
					</button>
				</li>
			</ul>
		</div>
	);
};
