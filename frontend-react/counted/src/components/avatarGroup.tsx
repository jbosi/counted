import { memo } from 'react';
import type { User } from '../types/users.model';
import { Avatar, type AvatarSize } from './avatar';

export interface AvatarGroupProps {
	data: User[];
	diplayUserLimit?: number;
	size?: AvatarSize;
}

const DEFAULT_DISPLAY_USER_LIMIT = 3;

export const AvatarGroup = memo(({ data, diplayUserLimit, size }: AvatarGroupProps) => {
	return (
		<>
			{data.slice(0, diplayUserLimit ?? DEFAULT_DISPLAY_USER_LIMIT).map((user) => (
				<Avatar key={user.id} name={user.name} size={size} />
			))}
			{data.length > (diplayUserLimit ?? DEFAULT_DISPLAY_USER_LIMIT) && <Avatar size={size} name={`+${data.length - DEFAULT_DISPLAY_USER_LIMIT}`} />}
		</>
	);
});
