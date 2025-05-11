import { type EventCallback, listen } from '@tauri-apps/api/event'

export function useEvent<T>(name: string, cb: EventCallback<T>) {
  const unListener = listen(name, cb)

  onUnmounted(async () => {
    const t = await unListener

    t()
  })
}
