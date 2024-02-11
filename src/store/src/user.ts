import { defineStore } from 'pinia'
import { Avatars, useAvatar, User } from '../../core'

// useStore 可以是 useUser、useCart 之类的任何东西
// 第一个参数是应用程序中 store 的唯一 id
export const user = defineStore('user', {
  // other options...
  state: () => {
    return {
      user: {} as User,
      isSignIn: false
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
    }
  }
})
