import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { AvatarModule } from 'primeng/avatar';

@Component({
    selector: 'app-expense',
    templateUrl: './expense.component.html',
    styleUrls: ['./expense.component.scss'],
    standalone: true,
    imports: [AvatarModule]
})
export class ExpenseComponent implements OnInit {
	constructor(
		private readonly activatedRoute: ActivatedRoute
	) {}
		
		ngOnInit(): void {
			this.activatedRoute.params.subscribe((p) => {
				const value = p as { projectId: number };
			})
		}
	}