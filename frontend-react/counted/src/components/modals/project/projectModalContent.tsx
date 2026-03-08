import type { UseMutationResult } from '@tanstack/react-query';
import { useCallback, useContext, useState, type ChangeEvent, type Dispatch, type RefObject, type SetStateAction } from 'react';
import { useFieldArray, type UseFormReturn } from 'react-hook-form';
import { CountedLocalStorageContext } from '../../../contexts/localStorageContext';
import { TrashIcon } from '../../../shared/icons/trashIcon';
import { UserIcon } from '../../../shared/icons/userIcon';
import type { CreatableProject, EditableProject, ProjectDto } from '../../../types/projects.model';
import { getProjectUserIdFromLocalstorage } from '../../../utils/get-project-from-localstorage';
import { ErrorValidationCallout } from '../../errorCallout';
import type { ProjectModalForm } from './models/projectModal.model';
import { ModalFooter } from '../shared/modalFooter';

export interface ProjectModalContentProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	onSubmit: (data: ProjectModalForm) => Promise<void>;
	mutationHook: UseMutationResult<ProjectDto, Error, CreatableProject, unknown> | UseMutationResult<ProjectDto, Error, EditableProject, unknown>;
	isSubmitLoading: boolean;
	selectedUserName: string | null;
	setSelectedUserName: Dispatch<SetStateAction<string | null>>;
	closeDialogFn: () => void;
	projectId?: string;
	useFormReturn: UseFormReturn<ProjectModalForm>;
}

export function ProjectModalContent({
	dialogRef,
	modalId,
	onSubmit,
	mutationHook,
	isSubmitLoading,
	selectedUserName,
	setSelectedUserName,
	projectId,
	closeDialogFn,
	useFormReturn,
}: ProjectModalContentProps) {
	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const errors = useFormReturn.formState.errors;
	const { error, isPending, isError } = mutationHook;

	const { fields, append, remove } = useFieldArray({
		control: useFormReturn.control,
		name: 'users',
	});

	const [newUserName, setNewUserName] = useState('');

	const handleAddUser = (event: ChangeEvent<HTMLInputElement>) => {
		setNewUserName(event.target.value);
	};

	const isUserSelected = useCallback(
		(field: { name: string; userId?: number }, projectId: string | undefined) => {
			const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

			return selectedUserName === field.name || (selectedUserName == null && storedUserId != null && storedUserId === field.userId);
		},
		[countedLocalStorage, selectedUserName],
	);

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => closeDialogFn()}>
						✕
					</button>
					<h1>{projectId ? 'Editer le projet' : 'Ajouter un projet'}</h1>
					<ErrorValidationCallout errors={useFormReturn.formState.errors} />
					<form className="ml-4 mr-4" onSubmit={useFormReturn.handleSubmit(onSubmit)}>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...useFormReturn.register('projectName')} />
							{errors.projectName && <span>Ce champ est requis</span>}

							<label className="label">Description</label>
							<input className="input w-full" {...useFormReturn.register('projectDescription')} />
							{errors.projectDescription && <span>Ce champ est requis</span>}

							{isPending && <span>Enregistrement…</span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

						<fieldset className="fieldset bg-base-200 border-base-300 rounded-box border p-4">
							<legend className="fieldset-legend">Liste des utilisateurs</legend>

							<div className="flex">
								<div className="flex-1">
									<label className="input w-full">
										<UserIcon />
										<input type="text" placeholder="Clark Kent" onChange={handleAddUser} value={newUserName} />
									</label>
								</div>
								<button
									type="button"
									className="btn btn-neutral join-item"
									onClick={() => {
										if (newUserName.trim()) {
											append({ name: newUserName.trim() });
											setNewUserName('');
										}
									}}
								>
									Ajouter
								</button>
							</div>

							<ul className="counted-list">
								{fields.map((field, index) => {
									return (
										<li key={field.id} className="projectDialogContent-userList">
											<button type="button" className="btn btn-square btn-sm p-1.5 btn-soft" onClick={() => remove(index)}>
												<TrashIcon />
											</button>
											<span className="self-center text-left text-sm">{field.name}</span>
											{isUserSelected(field, projectId) ? (
												<div className="badge badge-soft badge-accent self-center justify-self-center">Moi</div>
											) : (
												<button className="btn btn-outline btn-xs self-center" type="button" onClick={() => setSelectedUserName(field.name)}>
													C'est moi !
												</button>
											)}
										</li>
									);
								})}
							</ul>
						</fieldset>

						<ModalFooter closeDialogFn={closeDialogFn} isLoading={isSubmitLoading} />
					</form>
				</div>
			</dialog>
		</>
	);
}
