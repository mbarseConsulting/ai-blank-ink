import { APP_INITIALIZER, ApplicationConfig, provideBrowserGlobalErrorListeners } from '@angular/core';
import { AppConfigService, initAppConfig } from './app-config.service';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    AppConfigService,
    { provide: APP_INITIALIZER, useFactory: initAppConfig, deps: [AppConfigService], multi: true },
  ],
};
