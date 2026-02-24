import { useContext, useRef, useState } from 'react';
import { AppHeader } from '../../components/appHeader';
import { Dropdown } from '../../components/dropdowns/dropdown';
import { ImportTricountModal } from '../../components/modals/import/importTricountModal';
import { AddProjectModal } from '../../components/modals/project/addProjectModal';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { useTotalDebts } from '../../hooks/useExpenses';
import { useProjects } from '../../hooks/useProjects';
import { SettingsIcon } from '../../shared/icons/settingsIcon';
import { closeDialog, openDialog } from '../../utils/open-dialog';
import { ProjectItem } from './projectItem';

export function Projects() {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const { data: projects, isLoading, error } = useProjects(countedLocalStorage?.projects.map((p) => p.projectId) ?? []);
	const { totalDebts } = useTotalDebts(countedLocalStorage?.projects ?? []);
	const [isModalOpen, setIsProjectDialogOpen] = useState(false);
	const [isImportModalOpen, setIsImportModalOpen] = useState(false);
	const [showArchived, setShowArchived] = useState(false);

	const projectDialogRef = useRef<HTMLDialogElement>(null);
	const openProjectModal = () => openDialog(setIsProjectDialogOpen, projectDialogRef);
	const closeProjectModal = () => closeDialog(setIsProjectDialogOpen, projectDialogRef);

	const importDialogRef = useRef<HTMLDialogElement>(null);
	const openImportModal = () => openDialog(setIsImportModalOpen, importDialogRef);
	const closeImportModal = () => closeDialog(setIsImportModalOpen, importDialogRef);

	const visibleProjects = projects?.filter((p) => showArchived || p.status !== 'archived');

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
						<label className="label cursor-pointer justify-between gap-2 px-4">
							<span className="text-sm">Afficher projets archivés</span>
							<input type="checkbox" className="toggle toggle-sm toggle-primary" checked={showArchived} onChange={(e) => setShowArchived(e.target.checked)} />
						</label>
					</li>
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
				{!visibleProjects || visibleProjects.length === 0 ? (
					<div className="m-2">Ajoutez un projet en cliquant sur le bouton ci dessous</div>
				) : (
					visibleProjects.map((project) => <ProjectItem key={project.id} project={project} />)
				)}
			</div>

			<div className="flex gap-2 sticky mt-3 bottom-5 self-center">
				<button type="button" className="btn btn-circle btn-lg btn-primary" onClick={() => openProjectModal()}>
					+
				</button>
			</div>

			{isModalOpen && <AddProjectModal dialogRef={projectDialogRef} modalId={'AddProjectModal'} closeDialogFn={closeProjectModal} />}
			{isImportModalOpen && <ImportTricountModal dialogRef={importDialogRef} modalId={'ImportTricountModal'} closeDialogFn={closeImportModal} />}
		</div>
	);
}
