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
  filePath: string;
  archiveName: string;

  constructor(private route: ActivatedRoute, private backend: BackendService) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      this.id = params['id'];

      this.backend.getArchive(this.id).subscribe(archive => {
        this.archiveName = archive.name;
      });

      this.backend.getExplorerLevel(this.id, '/').subscribe(data => {
        this.data = data;
        this.loading = false;
      });
    });
  }

  handleItemClick(path: any) {
    this.filePath = path;
    this.loadingPreview = true;
    this.previewData = null;
    this.previewImage = null;

    if (path.endsWith('jpg') || path.endsWith('png') || path.endsWith('JPG') || path.endsWith('PNG')) {
      this.previewImage = this.backend.getFullUrl(this.id, path);
      this.loadingPreview = false;
      return;
    }

    this.backend.getExplorerLevel(this.id, path).subscribe(data => {
      this.loadingPreview = false;
      this.previewData = data;
    });
  }
}