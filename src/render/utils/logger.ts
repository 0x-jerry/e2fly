import { createSimpleLogger } from '@0x-jerry/utils'
import { isDev } from '../config'

export const logger = createSimpleLogger('[render]')

if (isDev) {
  logger.enable()
} else {
  logger.disable()
}
