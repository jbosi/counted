import { Component } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { ButtonModule } from 'primeng/button';

@Component({
	selector: 'app-sub-header',
	templateUrl: './sub-header.component.html',
	styleUrls: ['./sub-header.component.scss'],
	standalone: true,
	imports: [
		ButtonModule,
		AvatarModule
	],
})
export class SubHeaderComponent {}
