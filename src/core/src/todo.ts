import { TagProps } from 'element-plus'
import { Avatars } from './avatar'

/**优先级等级枚举 */
export enum PriorityEnum {
  Emergent = 'emergent',
  High = 'high',
  Mid = 'mid',
  Low = 'low'
}

/**优先级 */
export type Priority = {
  color: string
  name: PriorityEnum
}

/**优先级的颜色映射器 */
const PriorityColorMap = new Map<PriorityEnum, string>([
  [PriorityEnum.Emergent, '#E86D5E'],
  [PriorityEnum.High, '#F69D50'],
  [PriorityEnum.Mid, '#6CB6FF'],
  [PriorityEnum.Low, '#ADAC9A']
])

export const usePriorityColor = (priority: PriorityEnum): string => {
  return PriorityColorMap.get(priority) || '#ADAC9A'
}

/**用户类型 */
type User = {
  id: string
  name: string
  email: string
  avatar: Avatars
}

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
  [Status.NOT_START,"#ADAC9A"],
  [Status.IN_PROGRESS,"#56D4DD"],
  [Status.COMPLETED,"#8DDB80"],
  [Status.PENDING,"#8EBAC7"],

]);

export const useStatus = (status:Status):string=>{
  return StatusTypeMap.get(status)||"#ADAC9A";
}

type ITagProps = {
  type: 'info' | 'success' | 'warning' | ''|"danger"
  effect: 'dark' | 'light' | 'plain'
  label : string
}

/**待办实体类 */
export type ITODO = {
  id: string
  name: string
  priority: PriorityEnum
  /**审核人 */
  reviewers: User[]
  performers: User[]
  date: IDate
  tags: ITagProps[]
  status: Status
  description?: string
  information?: string
  /**附件 */
  annex?: any
}
