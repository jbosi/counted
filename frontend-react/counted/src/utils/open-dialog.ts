import type { RefObject, SetStateAction } from 'react';

export const openDialog = (setIsOpenFn: (setIsOpen: SetStateAction<boolean>) => void, dialogRef: RefObject<HTMLDialogElement | null>, delay?: number) => {
	setIsOpenFn(true);
	setTimeout(() => {
		dialogRef.current?.showModal();
	}, delay ?? 100);
};
