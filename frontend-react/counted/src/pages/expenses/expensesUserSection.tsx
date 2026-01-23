import { useRef, type RefObject } from 'react';
import { AvatarGroup } from '../../components/avatarGroup';
import { AddUserModal } from '../../components/modals/addUserModal';
import type { User } from '../../types/users.model';

interface ExpensesUserSectionProps {
	id: string;
	users: User[];
}

export const ExpensesUserSection = ({ id, users }: ExpensesUserSectionProps) => {
	const dialogRef = useRef<HTMLDialogElement>(null);

	return (
		<>
			<div className="flex p-4 justify-center">
				<div className="avatar-group -space-x-4 gap-1 items-center">
					<AvatarGroup data={users} size="w-12" diplayUserLimit={4} />
				</div>
				<button
					type="button"
					className="btn btn-circle btn-outline btn-lg self-center z-10"
					onClick={() => (dialogRef as RefObject<HTMLDialogElement>).current.showModal()}
				>
					+
				</button>
			</div>

			<AddUserModal dialogRef={dialogRef} modalId="addUserModal" projectId={id} currentUsers={users} />
		</>
	);
};
