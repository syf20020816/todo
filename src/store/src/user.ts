import { defineStore } from 'pinia'
import { Avatars, Todo, useAvatar, User } from '../../core'

// useStore 可以是 useUser、useCart 之类的任何东西
// 第一个参数是应用程序中 store 的唯一 id
export const user = defineStore('user', {
  // other options...
  state: () => {
    return {
      user: {} as User,
      isSignIn: false,
      todoList: [] as { label: string; value: number | string }[]
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

      this.todoList = [
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
    }
  }
})
