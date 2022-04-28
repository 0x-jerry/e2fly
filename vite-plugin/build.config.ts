import { Configuration } from 'electron-builder'

export const buildConfig: Configuration = {
  directories: {
    output: 'out',
  },
  // npmRebuild: false,
  // buildDependenciesFromSource: true,
  electronDownload: {
    // mirror: 'https://cdn.npmmirror.com/binaries/electron/',
  },
  files: ['dist/**/*'],
  nsis: {
    oneClick: false,
    allowElevation: true,
    allowToChangeInstallationDirectory: true,
    createDesktopShortcut: true,
    createStartMenuShortcut: true,
  },
}
