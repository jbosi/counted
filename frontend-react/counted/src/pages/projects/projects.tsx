import { useContext, useRef, useState } from 'react';
import { AppHeader } from '../../components/appHeader';
import { AddProjectModal } from '../../components/modals/project/addProjectModal';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { useTotalDebts } from '../../hooks/useExpenses';
import { useProjects } from '../../hooks/useProjects';
import { ProjectItem } from './projectItem';

export function Projects() {
	const dialogRef = useRef<HTMLDialogElement>(null);
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const { data: projects, isLoading, error } = useProjects(countedLocalStorage?.projects.map((p) => p.projectId) ?? []);
	const { totalDebts } = useTotalDebts(countedLocalStorage?.projects ?? []);
	const [isModalOpen, setIsModalOpen] = useState(false);

	const openModal = () => {
		setIsModalOpen(true);
		setTimeout(() => {
			dialogRef.current?.showModal();
		}, 100);
	};

	const closeModal = () => {
		setIsModalOpen(false);
		dialogRef.current?.close();
	};

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading projects: {error.message}</div>;
	}

	return (
		<div className="container overflow-auto app-container p-4">
			<AppHeader title="Counted" hideDropdown={true} />
			<div className="stats shadow overflow-visible">
				<div className="stat">
					<div className="stat-title">Nombre de Projets</div>
					<div className="stat-value">{countedLocalStorage?.projects?.length ?? 0}</div>
				</div>
				<div className="stat">
					<div className="stat-title">Total de mes dettes</div>
					<div className="stat-value">{totalDebts != null ? `${totalDebts.toFixed(2)} €` : '...'}</div>
				</div>
			</div>
			<div className="space-y-4">
				{!projects ? <div className="m-2">Vous n'avez aucun projet</div> : projects.map((project) => <ProjectItem key={project.id} project={project} />)}
			</div>

			<button type="button" className="btn btn-circle btn-lg self-center sticky mt-3 bottom-5 btn-primary" onClick={() => openModal()}>
				+
			</button>

			{isModalOpen && <AddProjectModal dialogRef={dialogRef} modalId={'AddProjectModal'} closeDialogFn={closeModal} />}
		</div>
	);
}
