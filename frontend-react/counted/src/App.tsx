import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { useCallback, useEffect, useState } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';
import './App.css';
import { ErrorFallback } from './components/errorFallback';
import { AuthContext } from './contexts/authContext';
import { CountedLocalStorageContext } from './contexts/localStorageContext';
import { computeProjectsToSync, removeFromLocalStorage, saveProjectEntry as saveProjectEntryFn, useInitializeLocalStorage } from './hooks/useLocalStorage';
import { ProjectLayout } from './layouts/projectLayout';
import { AccountPage } from './pages/auth/AccountPage';
import { LoginPage } from './pages/auth/LoginPage';
import { RegisterPage } from './pages/auth/RegisterPage';
import { PaymentPage } from './pages/payments/paymentList';
import { ProjectDetails } from './pages/projectDetails/projectsDetails';
import { Projects } from './pages/projects/projects';
import { accountProjectsService } from './services/accountProjectsService';
import { authService } from './services/authService';
import type { Account } from './types/auth.model';
import { type CountedLocalStorage, type CountedLocalStorageProject } from './types/localStorage.model';

const queryClient = new QueryClient({
	defaultOptions: { queries: { staleTime: 10_000 } },
});

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
		const controller = new AbortController();
		authService
			.me(controller.signal)
			.then((a) => setAccount(a ?? null))
			.catch(() => {
				if (!controller.signal.aborted) setAccount(null);
			});
		return () => controller.abort();
	}, []);

	// When account resolves as authenticated, load project associations from DB and sync any
	// anonymous localStorage projects that aren't on the server yet.
	useEffect(() => {
		if (!account) return;
		const controller = new AbortController();
		const localProjects = countedLocalStorage?.projects ?? [];
		accountProjectsService
			.getAll(controller.signal)
			.then(async (serverEntries) => {
				if (controller.signal.aborted) {
					return;
				}
				const toSync = computeProjectsToSync(localProjects, serverEntries);
				const acceptedIds = new Set(await accountProjectsService.upsertBatch(toSync));
				if (controller.signal.aborted) {
					return;
				}
				const synced = toSync.filter((p) => acceptedIds.has(p.projectId));
				setCountedLocalStorage({ projects: [...serverEntries, ...synced] });
			})
			.catch(() => {});
		return () => controller.abort();
	}, [account]); // eslint-disable-line react-hooks/exhaustive-deps

	const saveProjectEntry = useCallback(
		(entry: CountedLocalStorageProject) =>
			saveProjectEntryFn(!!account, entry, countedLocalStorage, setCountedLocalStorage, accountProjectsService.upsert.bind(accountProjectsService)),
		[account, countedLocalStorage, setCountedLocalStorage],
	);

	const removeProjectEntry = useCallback(
		async (projectId: string) => {
			if (account) {
				await accountProjectsService.remove(projectId);
				setCountedLocalStorage((prev) => ({
					projects: prev?.projects.filter((p) => p.projectId !== projectId) ?? [],
				}));
			} else {
				removeFromLocalStorage(countedLocalStorage, projectId, setCountedLocalStorage);
			}
		},
		[account, countedLocalStorage, setCountedLocalStorage],
	);

	return (
		<ErrorBoundary FallbackComponent={ErrorFallback}>
			<QueryClientProvider client={queryClient}>
				<AuthContext value={{ account, setAccount }}>
					<CountedLocalStorageContext value={{ countedLocalStorage, setCountedLocalStorage, saveProjectEntry, removeProjectEntry }}>
						<ReactQueryDevtools initialIsOpen={false} />
						<RouterProvider router={router} />
					</CountedLocalStorageContext>
				</AuthContext>
			</QueryClientProvider>
		</ErrorBoundary>
	);
}

export default App;
