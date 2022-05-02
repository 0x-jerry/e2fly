import SettingIcon from '~icons/ep/setting'
import DashboardIcon from '~icons/ic/outline-dashboard-customize'

import { Component } from 'vue'

export interface MenuItem {
  icon: Component
  text: string
  route: string
  onClick?: () => void
}

export const menus: MenuItem[] = [
  {
    icon: DashboardIcon,
    text: '配置',
    route: '/',
  },
  {
    icon: SettingIcon,
    text: '设置',
    route: '/setting',
  },
]
