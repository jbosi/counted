import { useRef, type RefObject } from 'react';
import { Avatar } from '../../components/avatar';
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
				<div className="avatar-group -space-x-4">
					{users.map((user) => (
						<Avatar key={user.id} name={user.name} size={'w-12'} />
					))}

					<button type="button" className="btn btn-circle btn-outline btn-lg self-center" onClick={() => (dialogRef as RefObject<HTMLDialogElement>).current.showModal()}>
						+
					</button>
				</div>
			</div>

			<AddUserModal dialogRef={dialogRef} modalId="addUserModal" projectId={id} currentUsers={users} />
		</>
	);
};
