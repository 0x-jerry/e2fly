import { EventCallback, listen } from '@tauri-apps/api/event'

export function useEvent<T>(name: string, cb: EventCallback<T>) {
  const unListener = listen(name, cb)

  onUnmounted(async () => {
    let t = await unListener

    t()
  })
}
