import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';
import { PatchModel } from './models';

@Component({
  selector: 'app-patches-panel',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './patches-panel.component.html',
  styleUrls: ['./patches-panel.component.css'],
})
export class PatchesPanelComponent {
  @Input() patches: PatchModel[] = [];
  @Output() selectPatch = new EventEmitter<PatchModel>();

  onSelect(patch: PatchModel): void {
    this.selectPatch.emit(patch);
  }
}

