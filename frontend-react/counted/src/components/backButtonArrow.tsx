import { useNavigate } from 'react-router';

interface BackButtonArrowProps {
	backButtonRoute: string;
}
export function BackButtonArrow({ backButtonRoute }: BackButtonArrowProps) {
	const navigate = useNavigate();

	return (
		<button className="btn btn-circle" onClick={() => navigate(backButtonRoute)}>
			<svg
				xmlns={'http://www.w3.org/2000/svg'}
				width={'24'}
				height={'24'}
				viewBox={'0 0 24 24'}
				fill={'none'}
				stroke={'currentColor'}
				strokeWidth={'2'}
				strokeLinecap={'round'}
				strokeLinejoin={'round'}
			>
				<polyline points="15 18 9 12 15 6" />
			</svg>
		</button>
	);
}
