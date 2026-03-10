import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { useEffect, useState } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';
import './App.css';
import { ErrorFallback } from './components/errorFallback';
import { AuthContext } from './contexts/authContext';
import { CountedLocalStorageContext } from './contexts/localStorageContext';
import { useInitializeLocalStorage } from './hooks/useLocalStorage';
import { ProjectLayout } from './layouts/projectLayout';
import { AccountPage } from './pages/auth/AccountPage';
import { LoginPage } from './pages/auth/LoginPage';
import { RegisterPage } from './pages/auth/RegisterPage';
import { PaymentPage } from './pages/payments/paymentList';
import { ProjectDetails } from './pages/projectDetails/projectsDetails';
import { Projects } from './pages/projects/projects';
import { authService } from './services/authService';
import type { Account } from './types/auth.model';
import { type CountedLocalStorage } from './types/localStorage.model';

const queryClient = new QueryClient();

const router = createBrowserRouter([
	{
		index: true,
		Component: Projects,
	},
	{
		path: '/login',
		Component: LoginPage,
	},
	{
		path: '/register',
		Component: RegisterPage,
	},
	{
		path: '/account',
		Component: AccountPage,
	},
	{
		path: '/projects/:projectId',
		Component: ProjectLayout,
		children: [
			{
				index: true,
				loader: ({ params }) => {
					return { projectId: params.projectId };
				},
				Component: ProjectDetails,
			},
			{
				path: 'expenses/:expenseId',
				Component: PaymentPage,
			},
		],
	},
]);

function App() {
	const [countedLocalStorage, setCountedLocalStorage] = useState<CountedLocalStorage>();
	const [account, setAccount] = useState<Account | null | undefined>(undefined);
	useInitializeLocalStorage(setCountedLocalStorage);

	useEffect(() => {
		authService
			.me()
			.then((a) => setAccount(a ?? null))
			.catch(() => setAccount(null));
	}, []);

	return (
		<ErrorBoundary FallbackComponent={ErrorFallback}>
			<QueryClientProvider client={queryClient}>
				<AuthContext value={{ account, setAccount }}>
					<CountedLocalStorageContext value={{ countedLocalStorage, setCountedLocalStorage }}>
						<ReactQueryDevtools initialIsOpen={false} />
						<RouterProvider router={router} />
					</CountedLocalStorageContext>
				</AuthContext>
			</QueryClientProvider>
		</ErrorBoundary>
	);
}

export default App;
