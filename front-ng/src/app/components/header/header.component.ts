import { Component } from '@angular/core';
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
export class HeaderComponent {}
