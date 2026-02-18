import { useContext, useRef, useState } from 'react';
import { AppHeader } from '../../components/appHeader';
import { ImportTricountModal } from '../../components/modals/import/importTricountModal';
import { AddProjectModal } from '../../components/modals/project/addProjectModal';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { useTotalDebts } from '../../hooks/useExpenses';
import { useProjects } from '../../hooks/useProjects';
import { ProjectItem } from './projectItem';
import { SettingsIcon } from '../../shared/icons/settingsIcon';
import { Dropdown } from '../../components/dropdowns/dropdown';

export function Projects() {
	const dialogRef = useRef<HTMLDialogElement>(null);
	const importDialogRef = useRef<HTMLDialogElement>(null);
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const { data: projects, isLoading, error } = useProjects(countedLocalStorage?.projects.map((p) => p.projectId) ?? []);
	const { totalDebts } = useTotalDebts(countedLocalStorage?.projects ?? []);
	const [isModalOpen, setIsModalOpen] = useState(false);
	const [isImportModalOpen, setIsImportModalOpen] = useState(false);

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

	const openImportModal = () => {
		setIsImportModalOpen(true);
		setTimeout(() => {
			importDialogRef.current?.showModal();
		}, 100);
	};

	const closeImportModal = () => {
		setIsImportModalOpen(false);
		importDialogRef.current?.close();
	};

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading projects: {error.message}</div>;
	}

	return (
		<div className="container overflow-auto app-container p-4">
			<AppHeader title="Counted">
				<Dropdown id="AppHeaderId" icon={<SettingsIcon />}>
					<li>
						<button type="button" className="btn btn-ghost" onClick={() => openImportModal()}>
							Importer depuis Tricount
						</button>
					</li>
				</Dropdown>
			</AppHeader>
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
				{!projects ? (
					<div className="m-2">Ajoutez un projet en cliquant sur le bouton ci dessous</div>
				) : (
					projects.map((project) => <ProjectItem key={project.id} project={project} />)
				)}
			</div>

			<div className="flex gap-2 sticky mt-3 bottom-5 self-center">
				<button type="button" className="btn btn-circle btn-lg btn-primary" onClick={() => openModal()}>
					+
				</button>
			</div>

			{isModalOpen && <AddProjectModal dialogRef={dialogRef} modalId={'AddProjectModal'} closeDialogFn={closeModal} />}
			{isImportModalOpen && <ImportTricountModal dialogRef={importDialogRef} modalId={'ImportTricountModal'} closeDialogFn={closeImportModal} />}
		</div>
	);
}
