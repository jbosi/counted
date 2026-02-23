export interface ModalFooterProps {
	closeDialogFn?: () => void;
	handleSubmit?: () => Promise<void>;
	hideSubmitButton?: boolean;
	hideCancelButton?: boolean;
	isLoading?: boolean;
	disabled?: boolean;
	submitName?: string;
}

export function ModalFooter({ isLoading, submitName, disabled, hideSubmitButton, hideCancelButton, closeDialogFn, handleSubmit }: ModalFooterProps) {
	return (
		<footer className="flex gap-1.5 mt-12 justify-end">
			{hideSubmitButton ? (
				<></>
			) : (
				<button className={`btn btn-primary ${isLoading ? 'loading' : ''}`} type="submit" disabled={disabled} onClick={handleSubmit}>
					{submitName !== undefined ? submitName : 'Enregistrer'}
				</button>
			)}
			{hideCancelButton ? (
				<></>
			) : (
				<button className="btn btn-outline" type="button" onClick={() => closeDialogFn?.()}>
					Annuler
				</button>
			)}
		</footer>
	);
}
