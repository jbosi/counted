import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
	selector: 'app-project',
	templateUrl: './project.component.html',
	styleUrls: ['./project.component.scss']
})
export class ProjectComponent implements OnInit {
	constructor(
		private readonly activatedRoute: ActivatedRoute
		) {}
		
		ngOnInit(): void {
			this.activatedRoute.params.subscribe((p) => {
				const value = p as { projectId: number };
			})
		}
	}