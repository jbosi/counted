import type { FallbackProps } from 'react-error-boundary';

export function ErrorFallback({ error, resetErrorBoundary }: FallbackProps) {
	return (
		<div role="alert" className="alert alert-error m-4 flex flex-col items-start gap-2">
			<p className="font-semibold">Une erreur est survenue</p>
			<p className="text-sm opacity-80">{error instanceof Error ? error?.message : String(error)}</p>
			<button type="button" className="btn btn-sm" onClick={resetErrorBoundary}>
				Réessayer
			</button>
		</div>
	);
}
