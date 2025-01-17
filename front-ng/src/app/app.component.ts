import { Component, OnInit, inject } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { HeaderComponent } from './components/header/header.component';
import { RouterParamService } from './modules';

@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    styleUrls: ['./app.component.scss'],
    imports: [HeaderComponent, RouterOutlet]
})
export class AppComponent implements OnInit {
	private readonly routerParamService = inject(RouterParamService);

	public title = 'front-ng';
		
	ngOnInit(): void {
		this.routerParamService.init();
	}
}