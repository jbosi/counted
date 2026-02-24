import { useRef, useState } from 'react';
import { useNavigate } from 'react-router';
import { AvatarGroup } from '../../components/avatarGroup';
import { EditProjectModal } from '../../components/modals/project/editProjectModal';
import { useDeleteProject, useUpdateProjectStatus } from '../../hooks/useProjects';
import { useUsersByProjectId } from '../../hooks/useUsers';
import type { ProjectDto, ProjectStatus } from '../../types/projects.model';
import { DropdownAction } from '../../components/dropdowns/dropdownAction';
import { Dropdown } from '../../components/dropdowns/dropdown';

export interface ProjectProps {
	project: ProjectDto;
}

function StatusBadge({ status }: { status: ProjectStatus }) {
	if (status === 'closed') {
		return (
			<div className="flex gap-1 items-center">
				<div className="status status-warning"></div>
				<span>Cloturé</span>
			</div>
		);
	}
	if (status === 'archived') {
		return (
			<div className="flex gap-1 items-center">
				<div className="status status-neutral"></div>
				<span>Archivé</span>
			</div>
		);
	}
	return (
		<div className="flex gap-1 items-center">
			<div className="status status-success"></div>
			<span>En cours</span>
		</div>
	);
}

export function ProjectItem({ project }: ProjectProps) {
	const { data: users, error, isLoading } = useUsersByProjectId(project.id);
	const { mutate: deleteProject } = useDeleteProject();
	const { mutate: updateStatus } = useUpdateProjectStatus(project.id);

	const [isProjectDialogOpen, setIsProjectDialogOpen] = useState(false);
	const projectDialogRef = useRef<HTMLDialogElement>(null);

	const openProjectModal = () => {
		setIsProjectDialogOpen(true);
		setTimeout(() => {
			projectDialogRef.current?.showModal();
		}, 100);
	};

	const closeProjectDialog = () => {
		setIsProjectDialogOpen(false);
		projectDialogRef.current?.close();
	};

	const navigate = useNavigate();

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading users: {error.message}</div>;
	}

	if (!users) {
		return <div>Error loading users</div>;
	}

	return (
		<>
			<section className="card bg-base-100 shadow-sm cursor-pointer" onClick={() => navigate(`projects/${project.id}`)}>
				<div className="card-body">
					<div className="flex flex-row justify-between">
						<h2 className="card-title">{project.name}</h2>
						<Dropdown id={project.id} icon={<>...</>}>
							<DropdownAction onDelete={() => deleteProject(project.id)} onEdit={() => openProjectModal()} />
							{project.status === 'ongoing' && (
								<>
									<li>
										<button
											className="btn btn-warning btn-soft"
											onClick={(e) => { e.stopPropagation(); updateStatus('closed'); }}
										>
											Cloturer
										</button>
									</li>
									<li>
										<button
											className="btn btn-neutral btn-soft"
											onClick={(e) => { e.stopPropagation(); updateStatus('archived'); }}
										>
											Archiver
										</button>
									</li>
								</>
							)}
							{project.status === 'closed' && (
								<>
									<li>
										<button
											className="btn btn-success btn-soft"
											onClick={(e) => { e.stopPropagation(); updateStatus('ongoing'); }}
										>
											Réouvrir
										</button>
									</li>
									<li>
										<button
											className="btn btn-neutral btn-soft"
											onClick={(e) => { e.stopPropagation(); updateStatus('archived'); }}
										>
											Archiver
										</button>
									</li>
								</>
							)}
							{project.status === 'archived' && (
								<li>
									<button
										className="btn btn-success btn-soft"
										onClick={(e) => { e.stopPropagation(); updateStatus('ongoing'); }}
									>
										Réouvrir
									</button>
								</li>
							)}
						</Dropdown>
						{isProjectDialogOpen && (
							<EditProjectModal dialogRef={projectDialogRef} modalId={'EditProjectModal'} project={project} users={users ?? []} closeDialogFn={closeProjectDialog} />
						)}
						{/* TODO handle errors */}
					</div>

					<p>{project.description}</p>

					<div className="card-actions justify-between">
						<StatusBadge status={project.status} />

						<div className="flex gap-1 items-center">
							<AvatarGroup data={users} />
						</div>
					</div>
				</div>
			</section>
		</>
	);
}
