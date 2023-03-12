import { Component, OnInit } from '@angular/core';
import { FetchHttpClient } from './modules';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
	public title = 'front-ng';

	constructor(
		private readonly http: FetchHttpClient
	) {}

	ngOnInit(): void {
		this.http.get('/projects');
	}
}
