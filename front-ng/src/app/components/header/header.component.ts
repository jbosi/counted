import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { RouterParamService } from '@hcount/modules';
import { ButtonModule } from 'primeng/button';

@Component({
	selector: 'app-header',
	templateUrl: './header.component.html',
	styleUrls: ['./header.component.scss'],
	standalone: true,
	imports: [
		ButtonModule
	]
})
export class HeaderComponent {
	constructor(private readonly routerParamService: RouterParamService) {}

	public navigateBackClick(): void {
		this.routerParamService.navigateRelative('..')
	}
}
