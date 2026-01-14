import { memo } from 'react';

export interface AvatarProps {
	name: string;
	size?: 'w-8' | 'w-10' | 'w-12';
	placeholderColor?: string;
}

const baseSize = 'w-8';

export const Avatar = memo((props: AvatarProps) => {
	const initials: string = props.name.slice(0, 2);
	const size: string = props.size ?? baseSize;
	const placeholderColor: string | undefined = props.placeholderColor;

	return (
		<div className="avatar avatar-placeholder">
			<div className={`bg-primary-content ${size} rounded-full`} style={{ backgroundColor: placeholderColor ?? defaultAvatarPlaceholderColor(initials) }}>
				<span className="text-xs">{initials}</span>
			</div>
		</div>
	);
});

function defaultAvatarPlaceholderColor(name: string): string {
	const hash = [...name.toLowerCase().trim()].reduce((h, c) => ((h << 5) + h + c.charCodeAt(0)) >>> 0, 5381);
	const hue = hash % 360;
	const sat = 50;
	const lig = 45;

	return `hsl(${hue}, ${sat}%, ${lig}%)`;
}
