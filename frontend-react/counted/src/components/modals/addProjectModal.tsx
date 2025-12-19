import type { RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddProject } from '../../hooks/useProjects';

export interface AddProjectModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
}

interface AddProjectModalForm {
	projectName: string;
	projectDescription: string;
}

export function AddProjectModal({ dialogRef, modalId }: AddProjectModalProps) {
	const {
		register,
		formState: { errors },
		getValues,
	} = useForm<AddProjectModalForm>();
	const { error, isPending, isError, mutate } = useAddProject();

	const onSubmit: SubmitHandler<AddProjectModalForm> = (data) => {
		mutate({ name: data.projectName, description: data.projectDescription });
		dialogRef.current?.close();
	};

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>Ajouter un projet</h1>
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
