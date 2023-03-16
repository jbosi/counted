import { Component, Input } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { IProjectCardViewModel } from '../../projects.component';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrls: ['./card.component.scss']
})
export class CardComponent {
	@Input() public project = {} as IProjectCardViewModel;

	constructor(
		private readonly router: Router,
		private readonly activatedRoute: ActivatedRoute
	) {}

	public onCardClick(): void {
		this.router.navigate([`${this.project.id}`], { relativeTo: this.activatedRoute });
	}
}
