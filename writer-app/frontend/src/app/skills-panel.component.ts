import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, Output } from '@angular/core';
import { SkillName } from './models';

export interface SkillDefinition {
  name: SkillName;
  label: string;
  description: string;
}

@Component({
  selector: 'app-skills-panel',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './skills-panel.component.html',
  styleUrls: ['./skills-panel.component.css'],
})
export class SkillsPanelComponent {
  @Input() skills: SkillDefinition[] = [];
  @Input() selectedSkill: SkillName | null = null;
  @Output() skillSelected = new EventEmitter<SkillName>();
  @Output() runSkillRequested = new EventEmitter<void>();

  onSelectSkill(name: SkillName): void {
    this.skillSelected.emit(name);
  }

  onRunSkill(): void {
    this.runSkillRequested.emit();
  }
}

