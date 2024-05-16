import { Component } from '@angular/core';
import { ArchiveInformation } from '../../interfaces/archive-information';
import { BackendService } from '../../services/backend.service';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.scss'
})
export class DashboardComponent {
  loading = true;
  archives: ArchiveInformation[] = [];

  constructor(private backend: BackendService) {}

  ngOnInit() {
    this.loadArchives();
    setInterval(() => {
        this.loadArchives();
    }, 2000);
  }

  loadArchives() {
    this.backend.getAllArchives().subscribe(archives => {
      this.loading = false;
      this.archives = archives;
    });
  }
}
