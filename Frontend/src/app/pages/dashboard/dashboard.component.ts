import { Component } from '@angular/core';
import { ArchiveInformation } from '../../interfaces/archive-information';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.scss'
})
export class DashboardComponent {
  loading = true;
  archives: ArchiveInformation[] = [];

  constructor() {}

  ngOnInit() {
    // add api call
    this.loading = false;
  }
}
