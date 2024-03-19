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
    this.backend.getAllArchives().subscribe(hashes => {
      hashes.forEach(hash => {
        this.backend.getArchive(hash).subscribe(archive => {
          this.loading = false;
          this.archives.push(archive);
        })
      })
    });
  }
}
