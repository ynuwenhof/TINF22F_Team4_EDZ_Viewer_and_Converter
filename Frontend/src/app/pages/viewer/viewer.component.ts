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

  constructor(private route: ActivatedRoute, private backend: BackendService) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const hash = params['hash'];
      const index = +params['index'];

      this.backend.getArchive(hash).subscribe(archive => {
        this.archive = archive;
      });

      this.backend.getPackage(hash, index).subscribe(pkg => {
        console.log(pkg);
        this.loading = false;
        this.package = pkg;
      });
    })
  }

  getObjectKeys(obj: any) {
    return Object.keys(obj);
  }
}
