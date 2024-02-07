/**
 * ============================================
 * @author:syf20020816@outlook.com
 * @since:20230223
 * @version:0.2.0
 * @type:ts
 * @description:vue-router设置页面路由地址
 * ============================================
 */

import User from '../views/user.vue'
import Plan from '../views/plan/plan.vue'
import Main from '../views/main.vue'
import History from '../views/history.vue'
import Collaborate from '../views/collaborate.vue'
import { RouteRecordRaw } from 'vue-router'
export const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    component: User
  },
  {
    path: '/plan',
    component: Plan
  },
  {
    path: '/main',
    component: Main
  },
  {
    path: '/history',
    component: History
  },
  {
    path: '/collaborate',
    component: Collaborate
  }
]
