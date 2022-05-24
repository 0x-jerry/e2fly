import { platform } from 'os'

export function isMac() {
  return platform() === 'darwin'
}

export function isWin() {
  return platform() === 'win32'
}

export function isLinux() {
  return !['win32', 'darwin'].includes(platform())
}
