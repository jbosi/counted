import { useContext } from 'react';
import { useNavigate } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { AuthContext } from '../../contexts/authContext';
import { useLogout } from '../../hooks/useAuth';

export function AccountPage() {
	const { account } = useContext(AuthContext);
	const navigate = useNavigate();
	const logout = useLogout();

	const handleLogout = async () => {
		await logout.mutateAsync();
		navigate('/');
	};

	if (!account) {
		navigate('/login');
		return null;
	}

	return (
		<div className="container app-container p-4">
			<AppHeader title="Mon compte" backButtonRoute="/" />

			<div className="card bg-base-100 shadow-xl mt-4">
				<div className="card-body space-y-4">
					<div>
						<p className="text-sm text-base-content/60">Nom</p>
						<p className="font-semibold text-lg">{account.displayName}</p>
					</div>
					<div>
						<p className="text-sm text-base-content/60">Email</p>
						<p className="font-semibold">{account.email}</p>
					</div>
					<div>
						<p className="text-sm text-base-content/60">Membre depuis</p>
						<p className="font-semibold">{Intl.DateTimeFormat().format(new Date(account.createdAt))}</p>
					</div>

					<div className="card-actions justify-end pt-2">
						<button type="button" className="btn btn-error btn-outline" onClick={handleLogout} disabled={logout.isPending}>
							{logout.isPending ? <span className="loading loading-spinner loading-sm" /> : 'Se déconnecter'}
						</button>
					</div>
				</div>
			</div>
		</div>
	);
}
