import { useProjects } from '../../hooks/useProjects';
import { Project } from './project';

export function Projects() {
	const { data: projects, isLoading, error } = useProjects();

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading projects: {error.message}</div>;
	}

	if (!projects) {
		return <div>Error loading projects</div>;
	}

	return (
		<div className="container p-4 max-w-md rounded-xl flex flex-col items-center">
			<h1 className="text-4xl font-light mb-10">Bonjour Jonathan</h1>

			<div className="space-y-4 min-w-md">
				{projects.map((project) => (
					<Project
						key={project.id}
						id={project.id}
						title={project.name}
						current_reimbursements={0}
						total_reimbursements={0}
						description={project.description ?? ''}
						currency={project.currency}
						created_at={project.created_at}
					/>
				))}
			</div>

			<button type="button" className="btn btn-circle btn-outline btn-lg">
				+
			</button>
		</div>
	);
}
