function deleteProjectById() {
	// TODO
}

export const DropdownButton = () => {
	const stopPropagation = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => e.stopPropagation();

	return (
		<div className="dropdown">
			<div tabIndex={0} role="button" className="btn btn-ghost btn-circle">
				...
			</div>
			<ul tabIndex={-1} className="dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm gap-1">
				<li>
					<button
						className="btn btn-ghost"
						onClick={(e) => {
							stopPropagation(e);
						}}
					>
						Editer
					</button>
				</li>
				<li>
					<button
						className="btn btn-ghost"
						onClick={(e) => {
							stopPropagation(e);
							deleteProjectById();
						}}
					>
						Supprimer
					</button>
				</li>
			</ul>
		</div>
	);
};
