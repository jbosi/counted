import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { ExpenseComponent } from './components/expense/expense.component';

@Component({
    selector: 'app-project',
    templateUrl: './project.component.html',
    styleUrls: ['./project.component.scss'],
    standalone: true,
    imports: [ExpenseComponent]
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