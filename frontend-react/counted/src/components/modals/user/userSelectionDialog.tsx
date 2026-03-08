import { useCallback, useContext, useState, type RefObject } from 'react';
import type { FieldErrors, FieldValues } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { addToLocalStorage } from '../../../hooks/useLocalStorage';
import type { CreatableUser, User } from '../../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import { ModalFooter } from '../shared/modalFooter';

export interface UserSelectionDialogProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	users: User[];
	closeDialogFn: () => void;
	projectId: string;
}

export function UserSelectionDialog({ dialogRef, modalId, users, projectId, closeDialogFn }: UserSelectionDialogProps) {
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const [selectedUserName, setSelectedUserName] = useState<string | null>(null);
	const [errors, setErrors] = useState<FieldErrors<FieldValues> | undefined>(undefined);

	const setCurrentUserForProject = useCallback(
		(userId: number, projectId: string) => {
			addToLocalStorage(countedLocalStorage, { projectId, userId }, setCountedLocalStorage);
		},
		[countedLocalStorage, setCountedLocalStorage],
	);

	const onSubmit = async (e: React.SubmitEvent<HTMLFormElement>): Promise<void> => {
		const selectedUserId = users?.find((u) => u.name === selectedUserName)?.id;
		if (selectedUserId != null) {
			setCurrentUserForProject(selectedUserId, projectId);
		} else {
			setErrors({ userSelection: { message: 'one user must be selected', type: 'error' } });
			e.preventDefault();
			return;
		}

		closeDialogFn();
	};

	const isUserSelected = useCallback(
		(u: User | CreatableUser, projectId: string | undefined) => {
			const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

			return selectedUserName === u.name || (selectedUserName == null && storedUserId && storedUserId === (u as User)?.id);
		},
		[countedLocalStorage, selectedUserName],
	);

	return (
		<dialog ref={dialogRef} id={modalId} className="modal">
			<div className="modal-box flex gap-3 flex-col">
				<h1>Quel utilisateur êtes vous ?</h1>
				<ErrorValidationCallout errors={errors} />
				<form className="ml-4 mr-4" onSubmit={(e) => onSubmit(e)}>
					<fieldset className="fieldset bg-base-200 border-base-300 rounded-box border p-4">
						<legend className="fieldset-legend">Liste des utilisateurs</legend>

						<ul className="counted-list">
							{users?.map((u, index) => {
								return (
									<li key={index} className="userSelectionDialog-userList">
										<span className="self-center text-left text-sm">{u.name}</span>
										{isUserSelected(u, projectId) ? (
											<div className="badge badge-soft badge-accent self-center justify-self-center">Moi</div>
										) : (
											<button className="btn btn-outline btn-xs self-center" type="button" onClick={() => setSelectedUserName(u.name)}>
												C'est moi !
											</button>
										)}
									</li>
								);
							})}
						</ul>
					</fieldset>

					<ModalFooter hideCancelButton={true} />
				</form>
			</div>
		</dialog>
	);
}
