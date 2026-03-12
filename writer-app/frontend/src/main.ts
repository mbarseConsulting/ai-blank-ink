import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import { AnalysisShellComponent } from './app/analysis-shell.component';

function bootstrap() {
  const isAnalysis = window.location.href.includes('#analysis');
  const root = isAnalysis ? AnalysisShellComponent : App;

  bootstrapApplication(root, appConfig).catch((err) =>
    console.error('Bootstrap Error:', err)
  );
}

bootstrap();
