import type { FieldErrors, FieldValues } from 'react-hook-form';

type ErrorCalloutProps = { errors: FieldErrors<FieldValues>; errorState?: never } | { errorState: string | null; errors?: never };

export function ErrorValidationCallout({ errors, errorState }: ErrorCalloutProps) {
	const errorMessages = getErrorMessages(errors, errorState);

	if (errorMessages.length === 0) {
		return null;
	}

	return (
		<div role="alert" className="alert alert-error whitespace-pre-line">
			<svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
				<path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<span>{errorMessages.join('\r\n')}</span>
		</div>
	);
}

function getErrorMessages(errors: FieldErrors<FieldValues> | undefined, errorState: string | null | undefined): string[] {
	if (errors != null) {
		return Object.entries(errors)
			.filter(([, e]) => e?.message)
			.map(([field, e]) => `Field: ${field} - error : ${e?.message}`);
	}
	if (errorState == null) {
		return [];
	}
	return JSON.parse(errorState)?.map((e: { path: string[]; message: string }) => `Field: ${e?.path?.[0]} - error : ${e?.message}`);
}
