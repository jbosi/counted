import { Component, inject } from '@angular/core';
import { RouterModule } from '@angular/router';
import { RouterParamService } from '@hcount/modules';
import { ButtonModule } from 'primeng/button';

@Component({
    selector: 'app-header',
    templateUrl: './header.component.html',
    styleUrls: ['./header.component.scss'],
    imports: [
        ButtonModule
    ]
})
export class HeaderComponent {
	private readonly routerParamService = inject(RouterParamService);


	public navigateBackClick(): void {
		this.routerParamService.navigateRelative('..')
	}
}
