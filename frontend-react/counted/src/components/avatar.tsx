import { memo } from 'react';

export interface AvatarProps {
	name: string;
	size?: 'w-8' | 'w-10' | 'w-12';
}

const baseSize = 'w-8';

export const Avatar = memo((props: AvatarProps) => {
	const initials: string = props.name.slice(0, 2);
	const size: string = props.size ?? baseSize;

	return (
		<div className="avatar avatar-placeholder">
			<div className={`bg-primary-content ${size} rounded-full`}>
				<span className="text-xs text-base-100">{initials}</span>
			</div>
		</div>
	);
});
