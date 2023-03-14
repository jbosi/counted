import { Component, Input } from '@angular/core';
import { IProjectCardViewModel } from '../../projects.component';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrls: ['./card.component.scss']
})
export class CardComponent {
	@Input() public project = {} as IProjectCardViewModel;
}
