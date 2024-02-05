import { InjectionToken } from "@angular/core";
import { IPrincipal } from "./principal.model";
import { getPrincipal } from "./principal.service";

export const PRINCIPAL = new InjectionToken<IPrincipal>('Currently logged user', {
	providedIn: 'root',
	factory: () => getPrincipal(),
});
