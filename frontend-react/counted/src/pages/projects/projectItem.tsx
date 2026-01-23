import { useCallback } from 'react';
import { useNavigate } from 'react-router';
import { AvatarGroup } from '../../components/avatarGroup';
import { useDeleteProject } from '../../hooks/useProjects';
import { useUsersByProjectId } from '../../hooks/useUsers';
import { DropdownButton } from './components/dropdown';

export interface ProjectProps {
	id: string;
	title: string;
	currentReimbursements: number;
	totalReimbursements: number;
	description?: string;
	currency: string;
	createdAt: string;
}

function getProgressPercentage(currentReimbursements: number, totalReimbursements: number): number {
	if (currentReimbursements === 0 || totalReimbursements === 0) {
		return 0;
	}

	return Math.round((currentReimbursements / totalReimbursements) * 100);
}

export function ProjectItem({ id, title, currentReimbursements: currentReimbursements, totalReimbursements: totalReimbursements, description }: ProjectProps) {
	const { data, error, isLoading } = useUsersByProjectId(id);
	const { mutate } = useDeleteProject();

	const navigate = useNavigate();

	const progressPercentage = useCallback(() => getProgressPercentage(currentReimbursements, totalReimbursements), [currentReimbursements, totalReimbursements]);

	if (isLoading) {
		return <div>Loading...</div>;
	}

	if (error) {
		return <div>Error loading users: {error.message}</div>;
	}

	if (!data) {
		return <div>Error loading users</div>;
	}

	return (
		<>
			<section className="card bg-base-100 shadow-sm cursor-pointer" onClick={() => navigate(`projects/${id}`)}>
				<div className="card-body">
					<div className="flex flex-row justify-between">
						<h2 className="card-title">{title}</h2>
						<DropdownButton id={id} onDelete={() => mutate(id)}>
							...
						</DropdownButton>{' '}
						{/* TODO handle errors */}
					</div>

					<p>{description}</p>

					{/* <div className="flex justify-between">
						<span>Remboursements</span>
						<span>
							{currentReimbursements}/{totalReimbursements}
						</span>
					</div>

					<progress className="progress" value={progressPercentage()} max={100}></progress> */}

					<div className="card-actions justify-between">
						<div className="flex gap-1 items-center">
							<div className="status status-success"></div>
							<span>En cours</span>
						</div>

						<div className="flex gap-1 items-center">
							<AvatarGroup data={data} />
						</div>
					</div>
				</div>
			</section>
		</>
	);
}
