import { useCallback } from 'react';
import { useNavigate } from 'react-router';
import { Avatar } from '../../components/avatar';
import { useDeleteProject } from '../../hooks/useProjects';
import { useUsersByProjectId } from '../../hooks/useUsers';
import { DropdownButton } from './components/dropdown';

export interface ProjectProps {
	id: string;
	title: string;
	current_reimbursements: number;
	total_reimbursements: number;
	description?: string;
	currency: string;
	created_at: string;
}

const DISPLAY_USER_LIMIT = 3;

function getProgressPercentage(current_reimbursements: number, total_reimbursements: number): number {
	if (current_reimbursements === 0 || total_reimbursements === 0) {
		return 0;
	}

	return Math.round((current_reimbursements / total_reimbursements) * 100);
}

export function ProjectItem({ id, title, current_reimbursements, total_reimbursements, description }: ProjectProps) {
	const { data, error, isLoading } = useUsersByProjectId(id);
	const { mutate } = useDeleteProject();

	const navigate = useNavigate();

	const progressPercentage = useCallback(() => getProgressPercentage(current_reimbursements, total_reimbursements), [current_reimbursements, total_reimbursements]);

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
			<section className="card bg-base-200 w-96 shadow-sm cursor-pointer" onClick={() => navigate(`projects/${id}`)}>
				<div className="card-body">
					<div className="flex flex-row justify-between">
						<h2 className="card-title">{title}</h2>
						<DropdownButton id={id} onDelete={() => mutate(id)} /> {/* TODO handle errors */}
					</div>

					<p>{description}</p>

					<div className="flex justify-between">
						<span>Remboursements</span>
						<span>
							{current_reimbursements}/{total_reimbursements}
						</span>
					</div>

					<progress className="progress" value={progressPercentage()} max={100}></progress>

					<div className="card-actions justify-between">
						<div className="flex gap-1 items-center">
							<div className="status status-success"></div>
							<span>En cours</span>
						</div>

						<div className="flex gap-1 items-center">
							{data.slice(0, DISPLAY_USER_LIMIT).map((user) => (
								<Avatar key={user.id} name={user.name} />
							))}
							{data.length > DISPLAY_USER_LIMIT && <Avatar name={`+${data.length - DISPLAY_USER_LIMIT}`} />}
						</div>
					</div>
				</div>
			</section>
		</>
	);
}
