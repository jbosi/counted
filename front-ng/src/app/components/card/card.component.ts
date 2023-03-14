import { Component, Input } from '@angular/core';
import { IProjectCardViewModel } from 'src/app/app.component';
import { IUser } from '../../modules';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrls: ['./card.component.scss']
})
export class CardComponent {
	@Input() public project = {} as IProjectCardViewModel;
}
