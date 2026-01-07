import { useContext, useRef, type RefObject } from 'react';
import { AddProjectModal } from '../../components/modals/project/addProjectModal';
import { useProjects } from '../../hooks/useProjects';
import { ProjectItem } from './projectItem';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';

export function Projects() {
	const dialogRef = useRef<HTMLDialogElement>(null);
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const { data: projects, isLoading, error } = useProjects(countedLocalStorage?.projects.map((p) => p.projectId) ?? []);

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading projects: {error.message}</div>;
	}

	return (
		<div className="container p-4 max-w-md rounded-xl flex flex-col">
			<h1 className="text-4xl font-light mb-10">Bonjour</h1>

			<div className="space-y-4">
				{!projects ? (
					<div className="m-2">Vous n'avez aucun projet</div>
				) : (
					projects.map((project) => (
						<ProjectItem
							key={project.id}
							id={project.id}
							title={project.name}
							currentReimbursements={0}
							totalReimbursements={0}
							description={project.description ?? ''}
							currency={project.currency}
							createdAt={project.createdAt}
						/>
					))
				)}
			</div>

			<button
				type="button"
				className="btn btn-circle btn-lg self-center sticky mt-3 bottom-5 btn-primary"
				onClick={() => (dialogRef as RefObject<HTMLDialogElement>).current.showModal()}
			>
				+
			</button>

			<AddProjectModal dialogRef={dialogRef} modalId={'AddProjectModal'} />
		</div>
	);
}
