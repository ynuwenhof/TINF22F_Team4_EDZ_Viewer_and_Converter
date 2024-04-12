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
    this.backend.getAllArchives().subscribe(archives => {
      for (const archive of archives) {
        this.loading = false;
        this.archives.push(archive);
      }
    });
  }
}
