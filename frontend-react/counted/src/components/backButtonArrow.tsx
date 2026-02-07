import { useNavigate } from 'react-router';
import { BackArrowIcon } from '../shared/icons/backArrowIcon';

interface BackButtonArrowProps {
	backButtonRoute: string;
}
export function BackButtonArrow({ backButtonRoute }: BackButtonArrowProps) {
	const navigate = useNavigate();

	return (
		<button className="btn btn-circle" onClick={() => navigate(backButtonRoute)}>
			<BackArrowIcon />
		</button>
	);
}
