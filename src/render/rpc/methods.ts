import { toArray, uuid } from '@0x-jerry/utils'
import { store } from '../store'

export type RendererThreadMethods = typeof methods

export const methods = {
  v2flyLog(str: string | string[]) {
    store.logs.unshift(
      ...toArray(str).map((n) => {
        return {
          id: uuid(),
          content: n,
        }
      })
    )

    // max length of log
    store.logs.length = store.logs.length > 1000 ? 1000 : store.logs.length
  },
}
