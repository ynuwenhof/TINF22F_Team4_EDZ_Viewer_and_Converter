import { Component, Input } from '@angular/core';
import { ArchiveInformation } from '../../interfaces/archive-information';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrl: './card.component.scss'
})
export class CardComponent {
  @Input({ required: true }) archive!: ArchiveInformation;
}

