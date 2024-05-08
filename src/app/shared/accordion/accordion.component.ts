import { Component, EventEmitter, Input, Output, input } from '@angular/core';
import { ExplorerItem } from '../../interfaces/explorer-item';
import { BackendService } from '../../services/backend.service';

@Component({
  selector: 'app-accordion',
  templateUrl: './accordion.component.html',
  styleUrl: './accordion.component.scss'
})
export class AccordionComponent {
  @Input() level: number;
  @Input() item: ExplorerItem;
  @Input() id: string;
  @Input() rootDir: string;
  @Output() itemClicked = new EventEmitter<any>();
  children: ExplorerItem[] = [];

  constructor(private backend: BackendService) {}

  onClick(item: ExplorerItem) {
    if (!item.is_dir) return this.itemClicked.emit(this.rootDir + item.name);
    if (this.children.length != 0) return this.children = [];

    this.backend.getExplorerLevel(this.id, `${this.rootDir}${item.name}`).subscribe(data => {
      this.children = data;
    });
  }
}
