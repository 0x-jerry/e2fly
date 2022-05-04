export type RendererThreadMethods = typeof methods

export const methods = {
  v2flyLog(str: string) {
    console.log('v2fly output:', str)
  },
}
