import type { UseMutationResult } from '@tanstack/react-query';
import { useCallback, useContext, useState, type ChangeEvent, type Dispatch, type RefObject, type SetStateAction } from 'react';
import { type FormState, type SubmitHandler, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { TrashIcon } from '../../../shared/icons/trashIcon';
import type { CreatableProject, EditableProject, ProjectDto } from '../../../types/projects.model';
import type { CreatableUser, User } from '../../../types/users.model';
import { ErrorValidationCallout } from '../../errorCallout';
import type { ProjectModalForm } from './models/projectModal.model';

export interface ProjectModalContentProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	register: UseFormRegister<ProjectModalForm>;
	onSubmit: SubmitHandler<ProjectModalForm>;
	getValues: UseFormGetValues<ProjectModalForm>;
	formState: FormState<ProjectModalForm>;
	mutationHook: UseMutationResult<ProjectDto, Error, CreatableProject, unknown> | UseMutationResult<ProjectDto, Error, EditableProject, unknown>;
	users: (CreatableUser | User)[];
	setUsers: Dispatch<SetStateAction<(CreatableUser | User)[]>>;
	projectErrorState: string | null;
	isSubmitLoading: boolean;
	selectedUserName: string | null;
	setSelectedUserName: Dispatch<SetStateAction<string | null>>;
	closeDialogFn: () => void;
	projectId?: string;
}

export function ProjectModalContent({
	dialogRef,
	modalId,
	onSubmit,
	getValues,
	register,
	formState,
	mutationHook,
	users,
	setUsers,
	projectErrorState,
	isSubmitLoading,
	selectedUserName,
	setSelectedUserName,
	projectId,
	closeDialogFn,
}: ProjectModalContentProps) {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const errors = formState.errors;
	const { error, isPending, isError } = mutationHook;

	const [newUser, setNewUser] = useState<CreatableUser>({ name: '', projectId: '' });

	const handleAddUser = (event: ChangeEvent<HTMLInputElement, HTMLInputElement>) => {
		setNewUser({ name: event.target.value, projectId: '' });
	};

	const isUserSelected = useCallback(
		(u: User | CreatableUser, projectId: string | undefined) => {
			const storedUserId = countedLocalStorage?.projects.find((p) => p.projectId === projectId)?.userId;

			return selectedUserName === u.name || (storedUserId && storedUserId === (u as User)?.id);
		},
		[countedLocalStorage?.projects, selectedUserName],
	);

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => closeDialogFn()}>
						✕
					</button>
					<h1>{projectId ? 'Editer le projet' : 'Ajouter un projet'}</h1>
					<ErrorValidationCallout errorState={projectErrorState} /> {/* TODO, use error boundary ? */}
					<form
						className="ml-4 mr-4"
						onSubmit={(e) => {
							e.preventDefault();
							onSubmit(getValues());
						}}
					>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('projectName', { required: true, maxLength: 100 })} />
							{errors.projectName && <span>Ce champ est requis</span>}

							<label className="label">Description</label>
							<input className="input w-full" {...register('projectDescription', { required: true, maxLength: 200 })} />
							{errors.projectDescription && <span>Ce champ est requis</span>}

							{isPending && <span>Enregistrement…</span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

						<fieldset className="fieldset bg-base-200 border-base-300 rounded-box border p-4">
							<legend className="fieldset-legend">Liste des utilisateurs</legend>

							<div className="flex">
								<div className="flex-1">
									<label className="input w-full">
										<svg className="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
											<g strokeLinejoin="round" strokeLinecap="round" strokeWidth="2.5" fill="none" stroke="currentColor">
												<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"></path>
												<circle cx="12" cy="7" r="4"></circle>
											</g>
										</svg>
										<input type="text" placeholder="Clark Kent" onChange={handleAddUser} value={newUser.name} />
									</label>
								</div>
								<button
									type="button"
									className="btn btn-neutral join-item"
									onClick={() => {
										setUsers([...users, newUser]);
										setNewUser({ name: '', projectId: '' });
									}}
								>
									Ajouter
								</button>
							</div>

							<ul className="flex flex-col gap-1">
								{users?.map((u, index) => {
									return (
										<li key={index} className="projectModalContent-userList">
											<span className="self-center text-left">{u.name}</span>
											<button
												type="button"
												className="btn btn-square btn-sm p-1.5 btn-soft"
												onClick={() => {
													setUsers(users?.filter((user) => user.name !== u.name));
												}}
											>
												<TrashIcon />
											</button>
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

						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className={`btn btn-primary ${isSubmitLoading ? 'loading' : ''}`} type="submit">
								Enregistrer
							</button>
							<button className="btn btn-outline" type="button" onClick={() => closeDialogFn()}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}
