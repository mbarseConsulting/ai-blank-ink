import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';

/** Mirrors writer-app/config.json (camelCase from backend). */
export interface AppConfig {
  paths: {
    storiesDir: string;
    analysisHistoryDir: string;
    envFile: string;
    skillsBaseDir: string;
  };
  window: {
    analysisLabel: string;
    analysisTitle: string;
    analysisDevUrl: string;
    analysisInnerSize: [number, number];
  };
  gemini: {
    defaultModel: string;
  };
  skills: {
    supportedForRun: string[];
    supportedForAnalysisDesk: string[];
    defaultOrder: string[];
    list: { name: string; label: string; description: string }[];
  };
  ui: {
    analysisInitDelayMs: number;
    skillFooterMaxHeightPercent: number;
    skillFooterMinHeightPx: number;
    skillFooterResizeMaxPercentOfWindow: number;
    editorMinHeightPx: number;
  };
  i18n: {
    emptyTreeMessage: string;
    analysisDeskTitle: string;
    analysisHistoryTitle: string;
    noReportPlaceholder: string;
    noReportHint: string;
    noDocSubtitle: string;
  };
}

@Injectable({ providedIn: 'root' })
export class AppConfigService {
  private config: AppConfig | null = null;

  async load(): Promise<AppConfig> {
    if (this.config) return this.config;
    this.config = await invoke<AppConfig>('get_app_config');
    return this.config;
  }

  get(): AppConfig {
    if (!this.config) {
      throw new Error('App config not loaded. Call load() first (e.g. in APP_INITIALIZER).');
    }
    return this.config;
  }

  getOrNull(): AppConfig | null {
    return this.config;
  }
}

export function initAppConfig(service: AppConfigService): () => Promise<unknown> {
  return () => service.load();
}
