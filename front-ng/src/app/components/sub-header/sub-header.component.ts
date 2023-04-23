import { Component } from '@angular/core';
import { RouterModule } from '@angular/router';
import { ButtonModule } from 'primeng/button';

@Component({
	selector: 'app-sub-header',
	templateUrl: './sub-header.component.html',
	styleUrls: ['./sub-header.component.scss'],
	standalone: true,
	imports: [
		ButtonModule,
		RouterModule
	],
})
export class SubHeaderComponent {}
