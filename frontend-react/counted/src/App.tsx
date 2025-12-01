import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import './App.css';
import { Projects } from './pages/projects/projects';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';

const router = createBrowserRouter([
	{
		path: '/',
		element: <Projects />,
	},
]);
const queryClient = new QueryClient();

function App() {
	return (
		<QueryClientProvider client={queryClient}>
			<RouterProvider router={router} />
		</QueryClientProvider>
	);
}

export default App;
