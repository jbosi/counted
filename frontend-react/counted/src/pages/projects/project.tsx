import { useState } from 'react';
import { Avatar } from '../../components/avatar';
import { useUsersByProjectId } from '../../hooks/useUsers';
import { DropdownButton } from './components/dropdown';
import { useNavigate } from 'react-router';
import { useDeleteProject } from '../../hooks/useProjects';

export interface ProjectProps {
	id: string;
	title: string;
	current_reimbursements: number;
	total_reimbursements: number;
	description?: string;
	currency: string;
	created_at: string;
}

function getProgressPercentage(current_reimbursements: number, total_reimbursements: number): number {
	if (current_reimbursements === 0 || total_reimbursements === 0) {
		return 0;
	}

	return Math.round((current_reimbursements / total_reimbursements) * 100);
}

export function Project(props: ProjectProps) {
	const [moreUsers, setMoreUsers] = useState<number>(0);
	const { data, error, isLoading } = useUsersByProjectId(props.id);
	const { mutate } = useDeleteProject();

	const navigate = useNavigate();

	const progressPercentage = getProgressPercentage(props.current_reimbursements, props.total_reimbursements);

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
			<section className="card bg-base-200 w-96 shadow-sm cursor-pointer" onClick={() => navigate(`projects/${props.id}`)}>
				<div className="card-body">
					<div className="flex flex-row justify-between">
						<h2 className="card-title">{props.title}</h2>
						<DropdownButton id={props.id} onDelete={() => mutate(props.id)} /> {/* TODO handle errors */}
					</div>

					<p>{props.description}</p>

					<div className="flex justify-between">
						<span>Remboursements</span>
						<span>
							{props.current_reimbursements}/{props.total_reimbursements}
						</span>
					</div>

					<progress className="progress" value={progressPercentage} max={100}></progress>

					<div className="card-actions justify-between">
						<div className="flex gap-1 items-center">
							<div className="status status-success"></div>
							<span>En cours</span>
						</div>

						<div className="flex gap-1 items-center">
							{data.slice(0, 3).map((user) => (
								<Avatar key={user.id} name={user.name} />
							))}
							{moreUsers > 0 && <Avatar name={`+${moreUsers}`} />}
						</div>
					</div>
				</div>
			</section>
		</>
	);
}
