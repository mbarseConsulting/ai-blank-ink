# Configuration centralisée (config.json)

**ID de référence :** `writer-app-config` — à citer dans les plans (INTERFACE_PLAN, ANALYSIS_WINDOW_PLAN, etc.) pour ne pas perdre l’info.

## Règle

**Toute valeur en dur (chaînes, chemins, listes de skills, constantes UI, textes i18n) doit vivre dans `writer-app/config.json`**, pas dans le code.

- **Backend (Rust)** : lire via `app_config()` (voir `src-tauri/src/lib.rs`).
- **Frontend (Angular)** : charger au démarrage via `AppConfigService`, utiliser `configService.get()`.

Quand tu ajoutes une nouvelle fonctionnalité qui nécessite une chaîne ou une constante, l’ajouter dans `config.json` et l’utiliser depuis le code.

## Fichier

- **Emplacement :** `writer-app/config.json`
- **Format :** JSON, clés en camelCase (aligné avec le frontend et la sérialisation Rust).

## Structure (référence)

| Section   | Rôle |
|----------|------|
| `paths`  | Chemins relatifs au workspace : `storiesDir`, `analysisHistoryDir`, `envFile`, `skillsBaseDir` |
| `window` | Fenêtre d’analyse : `analysisLabel`, `analysisTitle`, `analysisDevUrl`, `analysisInnerSize` |
| `gemini` | Modèle par défaut : `defaultModel` |
| `skills` | `supportedForRun`, `supportedForAnalysisDesk`, `defaultOrder`, `list` (name, label, description par skill) |
| `ui`     | Délais et tailles : `analysisInitDelayMs`, `skillFooterMaxHeightPercent`, `skillFooterMinHeightPx`, `skillFooterResizeMaxPercentOfWindow`, `editorMinHeightPx` |
| `i18n`   | Textes affichés : `emptyTreeMessage`, `analysisDeskTitle`, `analysisHistoryTitle`, `noReportPlaceholder`, `noReportHint`, `noDocSubtitle` |

Pour ajouter un nouveau champ : l’ajouter dans `config.json`, dans les structs Rust (`lib.rs`) si besoin, et côté frontend dans `AppConfigService` / interface `AppConfig` (`app-config.service.ts`).
