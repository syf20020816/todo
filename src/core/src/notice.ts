export type Notice = {
  title: string
  description: string
  notifier: string
  type: Notices
  data?: any
  date: string
  id: string
}

export type Notices = 'todo' | 'normal'
