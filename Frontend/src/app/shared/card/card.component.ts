import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrl: './card.component.scss'
})
export class CardComponent {
  @Input({ required: true }) title!: string;
  @Input({ required: true }) description!: string;
  @Input({ required: true }) size!: string;
  @Input({ required: true }) partCount!: number;
  @Input({ required: true }) date!: Date;
  @Input({ required: true }) id!: string;
}

