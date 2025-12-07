import { useState } from 'react';
import { Avatar } from '../../components/avatar';
import type { User } from '../../types/users.model';

interface ExpensesUserSectionProps {
	id: string;
	users: User[];
}

export const ExpensesUserSection = (props: ExpensesUserSectionProps) => {
	const [isUserModalOpen, setIsUserModalOpen] = useState(false);

	return (
		<>
			<div className="flex p-4 justify-center">
				<div className="avatar-group -space-x-4">
					{props.users.map((user) => (
						<Avatar key={user.id} name={user.name} size={'w-12'} />
					))}

					<button type="button" className="btn btn-circle btn-outline btn-lg self-center" onClick={() => setIsUserModalOpen(true)}>
						+
					</button>
				</div>
			</div>

			{/* <AddUserModal
				is_user_modal_open={isUserModalOpen}
				set_is_user_modal_open={setIsUserModalOpen}
				id={props.id}
			/> */}
		</>
	);
};
