import type { RefObject } from 'react';
import { type FormState, type SubmitHandler, type UseFormGetValues, type UseFormRegister } from 'react-hook-form';
import type { AddProjectModalForm } from './addProjectModal';
import type { EditProjectModalForm } from './editProjectModal';
import type { CreatableProject, EditableProject, ProjectDto } from '../../../types/projects.model';
import type { UseMutationResult } from '@tanstack/react-query';

export interface ProjectModalContentProps {
	isEdition: boolean;
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	register: UseFormRegister<AddProjectModalForm>;
	onSubmit: SubmitHandler<AddProjectModalForm | EditProjectModalForm>;
	getValues: UseFormGetValues<AddProjectModalForm | EditProjectModalForm>;
	formState: FormState<AddProjectModalForm | EditProjectModalForm>;
	mutationHook: UseMutationResult<ProjectDto, Error, CreatableProject, unknown> | UseMutationResult<ProjectDto, Error, EditableProject, unknown>;
}

export function ProjectModalContent({ isEdition, dialogRef, modalId, onSubmit, getValues, register, formState, mutationHook }: ProjectModalContentProps) {
	const errors = formState.errors;
	const { error, isPending, isError } = mutationHook;

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>{isEdition ? 'Editer le projet' : 'Ajouter un projet'}</h1>
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

						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className="btn btn-primary" type="submit">
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
