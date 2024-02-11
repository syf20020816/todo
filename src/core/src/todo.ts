import { TagProps } from 'element-plus'
import { Avatars } from './avatar'
import { Option } from './common'
import { Annex } from './annex'
import { User } from './user'

/**优先级等级枚举 */
export enum Priorities {
  Emergent = 'emergent',
  High = 'high',
  Mid = 'mid',
  Low = 'low'
}

/**优先级 */
export type Priority = {
  color: string
  name: Priorities
}

/**优先级的颜色映射器 */
const PriorityColorMap = new Map<Priorities, string>([
  [Priorities.Emergent, '#E86D5E'],
  [Priorities.High, '#F69D50'],
  [Priorities.Mid, '#6CB6FF'],
  [Priorities.Low, '#ADAC9A']
])

export const usePriorityColor = (priority: Priorities): string => {
  return PriorityColorMap.get(priority) || '#ADAC9A'
}

// /**用户类型 */
// type User = {
//   id: string
//   name: string
//   email: string
//   avatar: Avatars
// }

export type IDate = {
  start: string
  end: string
  during: number
}

/**待办状态 */
export enum Status {
  /**未开始 */
  NOT_START = 'not start',
  /**进行中 */
  IN_PROGRESS = 'in progress',
  /**已完成 */
  COMPLETED = 'completed',
  /**阻塞中 */
  PENDING = 'pending'
}

const StatusTypeMap = new Map<Status, string>([
  [Status.NOT_START, '#ADAC9A'],
  [Status.IN_PROGRESS, '#56D4DD'],
  [Status.COMPLETED, '#8DDB80'],
  [Status.PENDING, '#8EBAC7']
])

export const useStatus = (status: Status): string => {
  return StatusTypeMap.get(status) || '#ADAC9A'
}

type ITagProps = {
  type: 'info' | 'success' | 'warning' | '' | 'danger'
  effect: 'dark' | 'light' | 'plain'
  label: string
}

export type Todo = {
  id: string
  name: string
  priority: Priorities
  /// 审核人
  reviewers: Array<User>
  performers: Array<User>
  date: Date
  tags: Array<ITagProps>
  status: Status
  description: Option<String>
  information: Option<String>
  /// 附件
  annexs: Option<Array<Annex>>
  is_focus: boolean
}

export type TodoBox = {
  low: Array<Todo>
  mid: Array<Todo>
  // high 和 emergency都属于fatal
  fatal: Array<Todo>
  //关注
  focus: Array<Todo>
  history: Array<Todo>
}
