import { sleep } from '@0x-jerry/utils'

export function useInterval(cb: () => any, timeout = 200) {
  let wrapper = async () => {
    try {
      await cb()
    } catch (_) {}
  }

  let stoped = true

  const stop = () => {
    stoped = true
  }

  const run = async (): Promise<void> => {
    if (stoped) return

    await wrapper()
    await sleep(timeout)

    return run()
  }

  const start = () => {
    if (!stoped) return

    stoped = false

    run()
  }

  onMounted(() => start())

  onUnmounted(() => stop())

  return {
    start,
    stop
  }
}
