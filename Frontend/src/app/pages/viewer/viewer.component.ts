import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { PackageInformation } from '../../interfaces/package-information';
import { ArchiveInformation } from '../../interfaces/archive-information';

@Component({
  selector: 'app-viewer',
  templateUrl: './viewer.component.html',
  styleUrl: './viewer.component.scss'
})
export class ViewerComponent {
  loading = true;
  archive: ArchiveInformation;
  package: PackageInformation;

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const hash = params['hash'];
      const index = +params['index'];

      // add api call
      this.loading = false;
    })
  }
}
