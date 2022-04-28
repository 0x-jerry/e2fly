import { app } from 'electron'
import path from 'path'
import { isDev } from '../config'

/**
 * @example
 * ```ts
 * loadResource('dist/preload/index.js') // => <appRootDir>/dist/preload/index.js
 * ```
 * @param resourceRelativePath
 */
export function getResourcePath(resourceRelativePath: string) {
  const root = isDev ? process.cwd() : app.getAppPath()

  return path.join(root, resourceRelativePath)
}
