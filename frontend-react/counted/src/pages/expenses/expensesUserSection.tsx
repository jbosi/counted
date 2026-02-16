import { AvatarGroup } from '../../components/avatarGroup';
import type { User } from '../../types/users.model';

interface ExpensesUserSectionProps {
	users: User[];
}

export const ExpensesUserSection = ({ users }: ExpensesUserSectionProps) => {
	return (
		<>
			<div className="flex justify-center">
				<div className="avatar-group -space-x-4 gap-1 items-center">
					<AvatarGroup data={users} size="w-12" diplayUserLimit={4} />
				</div>
			</div>
		</>
	);
};
