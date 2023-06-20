import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { HeaderComponent } from './components/header/header.component';
import { RouterParamService } from './modules';

@Component({
	selector: 'app-root',
	templateUrl: './app.component.html',
	styleUrls: ['./app.component.scss'],
	standalone: true,
	imports: [HeaderComponent, RouterOutlet]
})
export class AppComponent implements OnInit {
	public title = 'front-ng';
	
	constructor(
		private readonly routerParamService: RouterParamService
	) {}
		
	ngOnInit(): void {
		this.routerParamService.init();
	}
}