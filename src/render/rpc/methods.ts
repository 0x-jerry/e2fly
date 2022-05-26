import { toArray } from '@0x-jerry/utils'
import { store } from '../store'

export type RendererThreadMethods = typeof methods

export const methods = {
  v2flyLog(str: string | string[]) {
    store.logs.unshift(...toArray(str))
  },
}
