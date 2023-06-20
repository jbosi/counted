/* eslint-disable no-restricted-syntax */
import { Injectable } from '@angular/core';
import { ActivatedRoute, ActivationEnd, NavigationEnd, Params, Router } from '@angular/router';
import { filter, map, Observable, ReplaySubject } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class RouterParamService {
	private params$ = new ReplaySubject<Params>(1);
	private data$ = new ReplaySubject<any>(1);

	constructor(
		private readonly activatedRoute: ActivatedRoute,
		private readonly router: Router
	) { }

	public init(): void {
		this.router.events.pipe(
			filter(e => e instanceof NavigationEnd),
			map(_ => this.getParams())
		).subscribe(v => this.params$.next(v));

		this.router.events.pipe(
			filter((event) => event instanceof ActivationEnd && event.snapshot.children?.length === 0),
		).subscribe((event) => {
			this.data$.next((event as any).snapshot?.data);
		});
	}

	public getParams(): Params {
		const params: Params = {};
		return this.getChild(params, this.activatedRoute.root);
	}

	public getParamsAsync(): Observable<Params> {
		return this.params$.asObservable();
	}

	public getDataAsync(): Observable<any> {
		return this.data$.asObservable();
	}

	public getParamAsync(param: string): Observable<string> {
		return this.params$.asObservable().pipe(
			filter(params => params.hasOwnProperty(param)),
			map(params => params[param])
		);
	}

	public getParam(param: string): string | undefined {
		if (this.getParams()?.hasOwnProperty(param)) {
			return this.getParams()[param];
		}
		return;
	}

	public getParsedParam(param: string): number | undefined {
		return parseIdFromRoute(this.getParam(param));
	}

	public getParsedParamAsync(param: string): Observable<number | undefined> {
		return this.getParamAsync(param).pipe(map(p => parseIdFromRoute(p)));
	}

	public getRouteSegmentByIndex(index: number): string {
		return this.router.url.split('/')[index + 1];
	}

	public navigateRelative(url: string): void {
		const segments = url?.split('/') ?? [];
		const forwardSegments = segments?.filter(s => s !== '..') ?? [];
		const backwardDepth = segments.length - forwardSegments.length;
		const currentUrl = this.router.routerState.snapshot.url;

		// Clean fragments
		const urlWithoutFragment = currentUrl?.split('#')?.[0];

		const newRoute = backwardDepth > 0
			? urlWithoutFragment.split('/').slice(0, -backwardDepth).concat(forwardSegments)
			: urlWithoutFragment.split('/').concat(forwardSegments);

		this.router.navigate(newRoute);
	}

	private getChild(params: Params, activatedRoute: ActivatedRoute | null): Params {
		if (activatedRoute == null) {
			return params;
		}
		const snapshotParams = activatedRoute.snapshot.params;
		if (Object.keys(snapshotParams).length !== 0) {
			params = { ...params, ...snapshotParams };
		}
		return this.getChild(params, activatedRoute.firstChild);
	}
}

export function parseIdFromRoute(routeId?: string): number | undefined {
	if (routeId == null) {
		return;
	}

	const id = parseInt(routeId, 10);
	if (isNaN(id)) {
		return;
	}
	
	return id;
}
