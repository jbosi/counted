import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import './App.css';
import { Projects } from './pages/projects/projects';

const queryClient = new QueryClient();

function App() {
	return (
		<QueryClientProvider client={queryClient}>
			<Projects></Projects>
		</QueryClientProvider>
	);
}

export default App;
