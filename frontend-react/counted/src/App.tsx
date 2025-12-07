import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import './App.css';
import { Projects } from './pages/projects/projects';
import { createBrowserRouter } from 'react-router';
import { RouterProvider } from 'react-router/dom';
import { ProjectDetails } from './pages/projectDetails/projectsDetails';

const router = createBrowserRouter([
	{
		path: '/',
		element: <Projects />,
	},
	{
		path: '/projects/:projectId',
		loader: ({ params }) => {
			return { projectId: params.projectId };
		},
		element: <ProjectDetails />,
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
