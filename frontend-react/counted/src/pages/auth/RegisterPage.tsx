import { zodResolver } from '@hookform/resolvers/zod';
import { useContext, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Link, useNavigate } from 'react-router';
import { z } from 'zod';
import { AuthContext } from '../../contexts/authContext';
import { useRegister } from '../../hooks/useAuth';

const REGISTER_SCHEMA = z.object({
	email: z.string().email('Email invalide'),
	displayName: z.string().min(1, 'Nom requis'),
	password: z.string().min(8, 'Le mot de passe doit contenir au moins 8 caractères'),
});

type RegisterForm = z.infer<typeof REGISTER_SCHEMA>;

export function RegisterPage() {
	const { account } = useContext(AuthContext);
	const navigate = useNavigate();
	const register_ = useRegister();

	const {
		register,
		handleSubmit,
		formState: { errors },
	} = useForm<RegisterForm>({ resolver: zodResolver(REGISTER_SCHEMA) });

	useEffect(() => {
		if (account) navigate('/');
	}, [account, navigate]);

	const onSubmit = async (data: RegisterForm) => {
		await register_.mutateAsync(data);
		navigate('/');
	};

	return (
		<div className="min-h-screen flex items-center justify-center p-4">
			<div className="card bg-base-100 shadow-xl w-full max-w-sm">
				<div className="card-body">
					<h2 className="card-title text-2xl font-bold justify-center mb-2">Créer un compte</h2>

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
							<label className="label" htmlFor="displayName">
								<span className="label-text">Nom affiché</span>
							</label>
							<input
								id="displayName"
								type="text"
								className={`input input-bordered w-full ${errors.displayName ? 'input-error' : ''}`}
								placeholder="Jean Dupont"
								{...register('displayName')}
							/>
							{errors.displayName && <span className="text-error text-sm mt-1">{errors.displayName.message}</span>}
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

						{register_.error && (
							<div className="alert alert-error text-sm py-2">
								<span>{register_.error.message.replace(/^Request failed with status \d+: /, '')}</span>
							</div>
						)}

						<button type="submit" className="btn btn-primary w-full" disabled={register_.isPending}>
							{register_.isPending ? <span className="loading loading-spinner loading-sm" /> : "S'inscrire"}
						</button>
					</form>

					<div className="divider text-sm">Déjà un compte ?</div>
					<Link to="/login" className="btn btn-ghost btn-sm">
						Se connecter
					</Link>
				</div>
			</div>
		</div>
	);
}
