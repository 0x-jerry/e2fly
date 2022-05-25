import type { E2FlyConfig } from '@/main/config'

export const store = reactive({
  config: {} as E2FlyConfig,
  logs: [] as string[],
})
