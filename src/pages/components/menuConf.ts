import ServerIcon from '~icons/carbon/bare-metal-server'
import DebugIcon from '~icons/carbon/debug'
import SettingIcon from '~icons/carbon/settings'

import { i18n } from '@/i18n'
import type { Component } from 'vue'

// @ts-ignore
const { t } = i18n.global

export interface MenuItem {
  icon: Component
  text: string
  route: string
  onClick?: () => void
}

export const menus: MenuItem[] = [
  {
    icon: DebugIcon,
    text: t('menu.log'),
    route: '/log',
  },
  {
    icon: SettingIcon,
    text: t('menu.setting'),
    route: '/setting',
  },
  {
    icon: ServerIcon,
    text: t('menu.server'),
    route: '/server',
  },
]
