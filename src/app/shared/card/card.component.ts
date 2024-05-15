import { Component, Input } from '@angular/core';
import { ArchiveInformation } from '../../interfaces/archive-information';
import { BackendService } from '../../services/backend.service';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrl: './card.component.scss'
})
export class CardComponent {
  @Input({ required: true }) archive!: ArchiveInformation;

  downloadLink: string;

  constructor(private backend: BackendService) {}

  ngOnInit() {
    this.downloadLink = this.backend.getDownloadLink(this.archive.id);
  }
}

