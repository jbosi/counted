import type { UseMutationResult } from '@tanstack/react-query';
import { useContext, useState, type ChangeEvent, type Dispatch, type RefObject, type SetStateAction } from 'react';
import { type FormState, type SubmitHandler, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import type { CreatableProject, EditableProject, ProjectDto } from '../../../types/projects.model';
import type { CreatableUser, User } from '../../../types/users.model';
import { ErrorValidationCallout } from '../../errorCallout';

export interface ProjectModalContentProps {
	isEdition: boolean;
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
}

export interface ProjectModalForm {
	projectName: string;
	projectDescription: string;
}

export function ProjectModalContent({
	isEdition,
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
}: ProjectModalContentProps) {
	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);
	const errors = formState.errors;
	const { error, isPending, isError } = mutationHook;

	const [newUser, setNewUser] = useState<CreatableUser>({ name: '', projectId: '' });

	const handleAddUser = (event: ChangeEvent<HTMLInputElement, HTMLInputElement>) => {
		setNewUser({ name: event.target.value, projectId: '' });
	};

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>{isEdition ? 'Editer le projet' : 'Ajouter un projet'}</h1>
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

							{users?.map((u, index) => {
								return (
									<div key={index} className="flex gap-3">
										<span className="self-center">{u.name}</span>
										<button
											type="button"
											className="btn btn-square btn-sm p-1.5 btn-soft"
											onClick={() => {
												setUsers(users?.filter((user) => user.name !== u.name));
											}}
										>
											<svg
												xmlns="http://www.w3.org/2000/svg"
												width="24"
												height="24"
												viewBox="0 0 24 24"
												fill="none"
												stroke="currentColor"
												strokeWidth="2"
												strokeLinecap="round"
												strokeLinejoin="round"
											>
												<path stroke="none" d="M0 0h24v24H0z" fill="none" />
												<path d="M4 7l16 0" />
												<path d="M10 11l0 6" />
												<path d="M14 11l0 6" />
												<path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" />
												<path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" />
											</svg>
										</button>
										{/* {countedLocalStorage?.projects.find((p) => p.projectId === projectId)?.userId === u.id ? (
											<div className="badge badge-soft badge-primary">Moi</div>
										) : (
											<button className="btn btn-outline btn-xs" type="button" onClick={() => setCurrentUserForProject(u.id, projectId)}>
												C'est moi !
											</button>
										)} */}
									</div>
								);
							})}
						</fieldset>

						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className={`btn btn-primary ${isSubmitLoading ? 'loading' : ''}`} type="submit">
								Enregistrer
							</button>
							<button className="btn btn-outline" type="button" onClick={() => dialogRef.current?.close()}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}
