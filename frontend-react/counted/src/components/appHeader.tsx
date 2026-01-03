import { BackButtonArrow } from './backButtonArrow';
import { Loading } from './loading';

export interface AppHeaderProps {
	title: string | undefined;
	backButtonRoute: string;
}

export function AppHeader({ title }: AppHeaderProps) {
	return (
		<div className="navbar px-0">
			<div className="navbar-start">
				<BackButtonArrow />
			</div>
			<div className="navbar-center">
				<h1 className="text-xl font-bold">{title ?? <Loading />}</h1>
			</div>
			<div className="navbar-end">
				<button className="btn btn-ghost btn-circle">
					<svg
						className={'w-6 h-6'}
						fill={'none'}
						stroke={'currentColor'}
						strokeWidth={'2'}
						strokeLinecap={'round'}
						strokeLinejoin={'round'}
						viewBox={'0 0 24 24'}
						path="M3 12h18M3 6h18M3 18h18"
					></svg>
				</button>
			</div>
		</div>
	);
}
