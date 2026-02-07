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
	const userLimit = diplayUserLimit ?? DEFAULT_DISPLAY_USER_LIMIT;

	return (
		<>
			{data.slice(0, userLimit).map((user) => (
				<Avatar key={user.id} name={user.name} size={size} /> // TODO: Should be a list ul ?
			))}
			{data.length > userLimit && <Avatar size={size} name={`+${data.length - userLimit}`} length={3} />}
		</>
	);
});
