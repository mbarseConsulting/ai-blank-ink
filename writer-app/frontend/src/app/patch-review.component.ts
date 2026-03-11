import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';
import { PatchModel } from './models';

@Component({
  selector: 'app-patch-review',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './patch-review.component.html',
  styleUrls: ['./patch-review.component.css'],
})
export class PatchReviewComponent {
  @Input() patch!: PatchModel | null;
  @Output() accept = new EventEmitter<PatchModel>();
  @Output() reject = new EventEmitter<PatchModel>();
  @Output() close = new EventEmitter<void>();

  onAccept() {
    if (this.patch) this.accept.emit(this.patch);
  }

  onReject() {
    if (this.patch) this.reject.emit(this.patch);
  }

  onClose() {
    this.close.emit();
  }
}

