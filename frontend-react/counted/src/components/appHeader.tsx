import type { ReactNode } from 'react';
import { BackButtonArrow } from './backButtonArrow';
import { Loading } from './loading';

export interface AppHeaderProps {
	title: string | undefined;
	backButtonRoute?: string;
	children?: ReactNode;
}

export function AppHeader({ title, backButtonRoute, children }: AppHeaderProps) {
	return (
		<div className="navbar px-0 gap-1">
			<div className="navbar-start flex-1">{backButtonRoute !== undefined ? <BackButtonArrow backButtonRoute={backButtonRoute} /> : <></>}</div>
			<h1 className="text-xl font-bold truncate">{title ?? <Loading />}</h1>
			<div className="navbar-end flex-1">{children}</div>
		</div>
	);
}
