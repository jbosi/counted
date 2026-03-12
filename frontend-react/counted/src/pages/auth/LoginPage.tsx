import { zodResolver } from '@hookform/resolvers/zod';
import { useContext, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Link, useNavigate } from 'react-router';
import { z } from 'zod';
import { AppHeader } from '../../components/appHeader';
import { AuthContext } from '../../contexts/authContext';
import { useLogin } from '../../hooks/useAuth';

const LOGIN_SCHEMA = z.object({
	email: z.string().email('Email invalide'),
	password: z.string().min(1, 'Mot de passe requis'),
});

type LoginForm = z.infer<typeof LOGIN_SCHEMA>;

export function LoginPage() {
	const { account } = useContext(AuthContext);
	const navigate = useNavigate();
	const login = useLogin();

	const {
		register,
		handleSubmit,
		formState: { errors },
	} = useForm<LoginForm>({ resolver: zodResolver(LOGIN_SCHEMA) });

	useEffect(() => {
		if (account) navigate('/');
	}, [account, navigate]);

	const onSubmit = async (data: LoginForm) => {
		await login.mutateAsync(data);
		navigate('/');
	};

	return (
		<div className="container overflow-auto app-container p-4">
			<div className="card bg-base-100 shadow-xl w-full h-full">
				<div className="card-body">
					<AppHeader title="Connexion" backButtonRoute="/" />

					<form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
						<div className="form-control">
							<label className="label" htmlFor="email">
								<span className="label-text">Email</span>
							</label>
							<input
								id="email"
								type="email"
								className={`input input-bordered w-full ${errors.email ? 'input-error' : ''}`}
								placeholder="vous@exemple.com"
								{...register('email')}
							/>
							{errors.email && <span className="text-error text-sm mt-1">{errors.email.message}</span>}
						</div>

						<div className="form-control">
							<label className="label" htmlFor="password">
								<span className="label-text">Mot de passe</span>
							</label>
							<input
								id="password"
								type="password"
								className={`input input-bordered w-full ${errors.password ? 'input-error' : ''}`}
								placeholder="••••••••"
								{...register('password')}
							/>
							{errors.password && <span className="text-error text-sm mt-1">{errors.password.message}</span>}
						</div>

						{login.error && (
							<div className="alert alert-error text-sm py-2">
								<span>{login.error.message.replace(/^Request failed with status \d+: /, '')}</span>
							</div>
						)}

						<button type="submit" className="btn btn-primary w-full" disabled={login.isPending}>
							{login.isPending ? <span className="loading loading-spinner loading-sm" /> : 'Se connecter'}
						</button>
					</form>

					<div className="divider text-sm">Pas encore de compte ?</div>
					<Link to="/register" className="btn btn-ghost btn-sm">
						Créer un compte
					</Link>
				</div>
			</div>
		</div>
	);
}
