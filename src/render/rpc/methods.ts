import { store } from '../store'

export type RendererThreadMethods = typeof methods

export const methods = {
  v2flyLog(str: string) {
    store.logs.push(str)
  },
}
