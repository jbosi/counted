import { memo, useState } from 'react';
import { useUsersByProjectId } from '../../hooks/useUsers';

export interface ProjectProps {
	id: string;
	title: string;
	current_reimbursements: number;
	total_reimbursements: number;
	description?: string;
	currency: string;
	created_at: string;
}

export interface User {
	id: string;
	name: string;
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
			<section className="card bg-base-200 w-96 shadow-sm cursor-pointer">
				<div className="card-body">
					{/* Title + dropdown actions */}
					<div className="flex flex-row justify-between">
						<h2 className="card-title">{props.title}</h2>
					</div>

					<p>{props.description}</p>

					{/* Reimbursements line */}
					<div className="flex justify-between">
						<span>Remboursements</span>
						<span>
							{props.current_reimbursements}/{props.total_reimbursements}
						</span>
					</div>

					{/* Progress bar */}
					<progress className="progress" value={progressPercentage} max={100}></progress>

					{/* Bottom actions */}
					<div className="card-actions justify-between">
						{/* Status badge */}
						<div className="flex gap-1 items-center">
							<div className="status status-success"></div>
							<span>En cours</span>
						</div>

						{/* Avatars */}
						<div className="flex gap-1 items-center">
							{data.slice(0, 3).map((user) => (
								<Avatar key={user.id} initials={user.name.slice(0, 2)} />
							))}
							{moreUsers > 0 && <Avatar initials={`+${moreUsers}`} />}
						</div>
					</div>
				</div>
			</section>
		</>
	);
}

type AvatarProps = { initials: string };
const Avatar = memo((props: AvatarProps) => {
	return (
		<div className="avatar avatar-placeholder">
			<div className="bg-neutral text-neutral-content rounded-full w-8">{props.initials}</div>
		</div>
	);
});
