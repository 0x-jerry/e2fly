import { createSimpleLogger } from '@0x-jerry/utils'

export const logger = createSimpleLogger('[render]')

if (import.meta.env.DEV) {
  logger.enable()
} else {
  logger.disable()
}
