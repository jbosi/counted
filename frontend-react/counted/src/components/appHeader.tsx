import { DropdownButton, type DropdownButtonProps } from '../pages/projects/components/dropdown';
import { BackButtonArrow } from './backButtonArrow';
import { Loading } from './loading';

export interface AppHeaderProps extends Pick<DropdownButtonProps, 'onDelete' | 'onEdit'> {
	title: string | undefined;
	backButtonRoute?: string;
	hideDropdown?: boolean;
}

export function AppHeader({ title, onEdit, onDelete, backButtonRoute, hideDropdown }: AppHeaderProps) {
	return (
		<div className="navbar px-0 gap-1">
			<div className="navbar-start flex-1">{backButtonRoute !== undefined ? <BackButtonArrow backButtonRoute={backButtonRoute} /> : <></>}</div>
			<h1 className="text-xl font-bold truncate">{title ?? <Loading />}</h1>
			<div className="navbar-end flex-1">
				{hideDropdown ? (
					<></>
				) : (
					<DropdownButton id="AppHeaderId" onEdit={onEdit} onDelete={onDelete}>
						<svg className="w-6 h-6" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" viewBox="0 0 24 24">
							<path d="M3 12h18M3 6h18M3 18h18"></path>
						</svg>
					</DropdownButton>
				)}
			</div>
		</div>
	);
}
