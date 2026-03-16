import { createEvent, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import type { User } from '../../../types/users.model';
import { UserSelectionDialog } from './userSelectionDialog';

const users: User[] = [
	{ id: 1, name: 'Alice' },
	{ id: 2, name: 'Bob' },
];

function renderDialog(saveProjectEntry = vi.fn().mockResolvedValue(undefined), closeDialogFn = vi.fn()) {
	const dialogRef = { current: null };
	render(
		<CountedLocalStorageContext
			value={{ countedLocalStorage: undefined, setCountedLocalStorage: vi.fn(), saveProjectEntry, removeProjectEntry: vi.fn() }}
		>
			<UserSelectionDialog modalId="test" projectId="project-1" users={users} dialogRef={dialogRef} closeDialogFn={closeDialogFn} />
		</CountedLocalStorageContext>,
	);
	return { saveProjectEntry, closeDialogFn };
}

describe('UserSelectionDialog', () => {
	it('prevents default form submission when a user is selected', async () => {
		const { saveProjectEntry, closeDialogFn } = renderDialog();

		fireEvent.click(screen.getByText('Alice'));

		const form = document.querySelector('form')!;
		const submitEvent = createEvent.submit(form);
		fireEvent(form, submitEvent);

		expect(submitEvent.defaultPrevented).toBe(true);
		await waitFor(() => {
			expect(saveProjectEntry).toHaveBeenCalledWith({ projectId: 'project-1', userId: 1 });
			expect(closeDialogFn).toHaveBeenCalled();
		});
	});

	it('does not call closeDialogFn when no user is selected', async () => {
		const { closeDialogFn } = renderDialog();

		const form = document.querySelector('form')!;
		fireEvent.submit(form);

		await waitFor(() => expect(closeDialogFn).not.toHaveBeenCalled());
		expect(screen.getByText('userSelection: one user must be selected')).toBeDefined();
	});
});
