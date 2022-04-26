import { Configuration } from 'electron-builder'

export const buildConfig: Configuration = {
  directories: {
    output: 'dist/electron',
  },
  npmRebuild: false,
  buildDependenciesFromSource: true,
  electronDownload: {
    mirror: 'https://npm.taobao.org/mirrors/electron/',
  },
  files: ['dist/main/**/*', 'dist/render/**/*'],
  nsis: {
    oneClick: false,
    allowElevation: true,
    allowToChangeInstallationDirectory: true,
    createDesktopShortcut: true,
    createStartMenuShortcut: true,
  },
}
