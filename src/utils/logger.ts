import { createLogger } from '@0x-jerry/utils'
import { isDev } from '../config'

export const logger = createLogger('[render]')

if (isDev) {
  logger.enable()
} else {
  logger.disable()
}
