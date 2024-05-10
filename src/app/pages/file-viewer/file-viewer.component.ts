import { Component } from '@angular/core';
import { ArchiveInformation } from '../../interfaces/archive-information';
import { ActivatedRoute } from '@angular/router';
import { BackendService } from '../../services/backend.service';
import { ExplorerItem } from '../../interfaces/explorer-item';

@Component({
  selector: 'app-file-viewer',
  templateUrl: './file-viewer.component.html',
  styleUrl: './file-viewer.component.scss'
})
export class FileViewerComponent {
  loading = true;
  loadingPreview = false;
  previewData: any;
  previewImage: string | null = null;
  data: ExplorerItem[];
  id: string;

  constructor(private route: ActivatedRoute, private backend: BackendService) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.id = params['id'];

      this.backend.getExplorerLevel(this.id, '/').subscribe(data => {
        this.data = data;
        this.loading = false;
      });
    });
  }

  handleItemClick(path: any) {
    this.loadingPreview = true;
    this.previewData = null;
    this.previewImage = null;

    if (path.endsWith('jpg') || path.endsWith('png')) {
      this.previewImage = this.backend.getImageUrl(this.id, path);
      this.loadingPreview = false;
      return;
    }

    this.backend.getExplorerLevel(this.id, path).subscribe(data => {
      console.log(data);
      this.loadingPreview = false;
      this.previewData = data;
    });
  }
}