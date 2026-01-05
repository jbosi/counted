import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { useState } from 'react';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';
import './App.css';
import { CountedLocalStorageContext } from './contexts/localStorageContext';
import { useInitializeLocalStorage } from './hooks/useLocalStorage';
import { ExpenseLayout } from './layouts/expenseLayout';
import { ProjectLayout } from './layouts/projectLayout';
import { PaymentPage } from './pages/payments/paymentList';
import { ProjectDetails } from './pages/projectDetails/projectsDetails';
import { Projects } from './pages/projects/projects';
import { type CountedLocalStorage } from './types/localStorage.model';

const queryClient = new QueryClient();

const router = createBrowserRouter([
	{
		index: true,
		Component: Projects,
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
				Component: ExpenseLayout,
				children: [
					{
						index: true,
						Component: PaymentPage,
					},
				],
			},
		],
	},
]);

function App() {
	const [countedLocalStorage, setCountedLocalStorage] = useState<CountedLocalStorage>();
	useInitializeLocalStorage(setCountedLocalStorage);

	return (
		<QueryClientProvider client={queryClient}>
			<CountedLocalStorageContext value={{ countedLocalStorage, setCountedLocalStorage }}>
				<ReactQueryDevtools initialIsOpen={false} />
				<RouterProvider router={router} />
			</CountedLocalStorageContext>
		</QueryClientProvider>
	);
}

export default App;
