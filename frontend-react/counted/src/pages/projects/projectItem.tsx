import { useRef, useState } from 'react';
import { useNavigate } from 'react-router';
import { AvatarGroup } from '../../components/avatarGroup';
import { EditProjectModal } from '../../components/modals/project/editProjectModal';
import { useDeleteProject } from '../../hooks/useProjects';
import { useUsersByProjectId } from '../../hooks/useUsers';
import type { ProjectDto } from '../../types/projects.model';
import { DropdownAction } from '../../components/dropdowns/dropdownAction';

export interface ProjectProps {
	project: ProjectDto;
}

export function ProjectItem({ project }: ProjectProps) {
	const { data: users, error, isLoading } = useUsersByProjectId(project.id);
	const { mutate } = useDeleteProject();

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
						<DropdownAction id={project.id} onDelete={() => mutate(project.id)} onEdit={() => openProjectModal()} icon={<>...</>} />
						{isProjectDialogOpen && (
							<EditProjectModal dialogRef={projectDialogRef} modalId={'EditProjectModal'} project={project} users={users ?? []} closeDialogFn={closeProjectDialog} />
						)}
						{/* TODO handle errors */}
					</div>

					<p>{project.description}</p>

					<div className="card-actions justify-between">
						<div className="flex gap-1 items-center">
							<div className="status status-success"></div>
							<span>En cours</span>
						</div>

						<div className="flex gap-1 items-center">
							<AvatarGroup data={users} />
						</div>
					</div>
				</div>
			</section>
		</>
	);
}
