import SettingIcon from '~icons/ep/setting'
import ServerIcon from '~icons/carbon/bare-metal-server'
import DebugIcon from '~icons/carbon/debug'

import { Component } from 'vue'

export interface MenuItem {
  icon: Component
  text: string
  route: string
  onClick?: () => void
}

export const menus: MenuItem[] = [
  {
    icon: DebugIcon,
    text: '日志',
    route: '/log',
  },
  {
    icon: SettingIcon,
    text: '设置',
    route: '/setting',
  },
  {
    icon: ServerIcon,
    text: '服务器',
    route: '/',
  },
]
