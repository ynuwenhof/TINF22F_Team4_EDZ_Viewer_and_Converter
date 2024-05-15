import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { PackageInformation } from '../../interfaces/package-information';
import { ArchiveInformation } from '../../interfaces/archive-information';
import { BackendService } from '../../services/backend.service';

@Component({
  selector: 'app-viewer',
  templateUrl: './viewer.component.html',
  styleUrl: './viewer.component.scss'
})
export class ViewerComponent {
  loading = true;
  archive: ArchiveInformation;
  package: PackageInformation;
  imageUrl: string;

  constructor(private route: ActivatedRoute, private backend: BackendService) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const id = params['id'];
      const index = +params['index'];

      this.backend.getArchive(id).subscribe(archive => {
        this.archive = archive;
      });

      this.backend.getPackage(id, index).subscribe(pkg => {
        this.loading = false;
        this.package = pkg;
        this.imageUrl = this.backend.getFullUrl(id, pkg.image);
      });
    })
  }

  getObjectKeys(obj: any) {
    return Object.keys(obj);
  }
}
