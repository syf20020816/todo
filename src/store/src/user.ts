import { defineStore } from 'pinia'
import { Avatars, Notice, Status, Todo, useAvatar, User } from '../../core'
import api from '../../api'
import { ElMessage, ElNotification } from 'element-plus'

// useStore 可以是 useUser、useCart 之类的任何东西
// 第一个参数是应用程序中 store 的唯一 id
export const user = defineStore('user', {
  // other options...
  state: () => {
    return {
      user: {} as User,
      isSignIn: false,
      todoInfoList: [] as { label: string; value: number | string }[],
      todos: [] as Todo[],
      watcher: null as null | number,
      msgBox: [] as Notice[]
    }
  },
  actions: {
    useAvatar,
    checkSetIsSignIn() {
      console.log('check')
      let flag = window.localStorage.getItem('todo-sign-in')
      // null 或 空字符串 或 undefined的情况下设置false
      if (flag ?? false) {
        this.isSignIn = true
      } else {
        this.isSignIn = false
      }
    },
    setUser(user: User) {
      this.user = user
      window.localStorage.setItem('todo-user', JSON.stringify(this.user))
      this.updateTodoList()
      this.todoWatcher()
    },
    setSignIn() {
      window.localStorage.setItem('todo-sign-in', this.user.username.toString())
      this.isSignIn = true
    },
    getUsername() {
      let username = this.user.username ?? window.localStorage.getItem('todo-sign-in')
      if (username) {
        return username
      } else {
        // 丢失了username的情况说明需要下线登录
        this.logout()
      }
    },
    logout() {
      window.localStorage.clear()
      this.isSignIn = false
      this.user = {} as User
      this.msgBox = []
    },
    updateTodoList() {
      let { todos, todoNumber } = this.user
      let { low, mid, fatal, focus } = todos
      const today = new Date(new Date().toLocaleDateString()).getTime()
      //检查每个todo的开始时间
      const countIsToday = (todos: Todo[]): number => {
        return todos.filter(todo => {
          let start = new Date(todo.date.start)
          let startDate = new Date(start.toLocaleDateString()).getTime()
          return startDate == today
        }).length
      }
      const totalToday = countIsToday(low) + countIsToday(mid) + countIsToday(fatal)

      this.todoInfoList = [
        {
          label: 'TODOs for today',
          value: totalToday
        },
        {
          label: 'Emergent TODOs',
          value: fatal.length + focus.length
        },
        {
          label: 'Normal TODOs',
          value: low.length + mid.length
        },
        {
          label: 'All TODOs',
          value: low.length + mid.length + fatal.length
        }
      ]

      this.todos = [...low, ...mid, ...fatal]
    },
    notifyTodoSys(todo: Todo) {
      let id = 'todo-' + todo.id!
      let flag = this.msgBox.filter(item => item.id === id).length === 0
      if (flag) {
        this.msgBox.push({
          title: 'Notifications: Todo-' + todo.name,
          description: 'The current TODO has timed out, please choose a processing strategy',
          notifier: 'System',
          type: 'todo',
          data: todo,
          date: new Date().toLocaleString(),
          id
        })
      }
    },
    todoWatcher() {
      if (this.watcher !== null) {
        clearInterval(this.watcher)
      }
      this.watcher = setInterval(() => {
        let current = new Date().getTime()
        this.todos.forEach(async (todo: Todo) => {
          let { date, status, id } = todo
          if (status !== Status.COMPLETED && status !== Status.PENDING) {
            // 更新状态
            // > 0 已经开始
            let start = current - new Date(date.start).getTime()
            if (start > 0) {
              if (status !== Status.IN_PROGRESS) {
                todo.status = Status.IN_PROGRESS
                // 更新
                const data = await api.todo.updateTodoStatus(id!, todo.status)
                if (data) {
                  const user = await api.user.getUserInfo(this.user.username)
                  this.setUser(user!)
                } else {
                  ElMessage({
                    type: 'error',
                    message: 'User Update Error!'
                  })
                }
              }
            }

            let remain = new Date(date.end).getTime() - current
            if (remain < 0) {
              //已经结束了
              //通知系统提醒用户，若用户同意，将TODO设置为Pending状态
              // const data = api.todo.unCompletedTodo(this.user.username, todo)
              this.notifyTodoSys(todo)
            }
          }
        })
      }, 15 * 1000)
    }
  }
})
