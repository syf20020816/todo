/**
 * 用户相关API
 * - 登录
 * - 注册
 * - 获取用户信息
 * - 等
 */
import { Request } from '../axios/index'
import { User, UserLoginForm, UserInfoChangeForm, Avatars } from '../../core'
import type { ApiResponse } from './type'

const request = Request.init()

export const signin = async (params: UserLoginForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/signin/', params)
  return data
}

export const signup = async (params: UserLoginForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/signup/', params)
  return data
}

export const getUserInfo = async (username: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get('/user/info/' + username)
  return data
}

export const setUserInfo = async (username: string, params: UserInfoChangeForm): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/user/info/' + username, params)
  return data
}

export const setUserAvatar = async (username: string, avatar: Avatars): Promise<boolean> => {
  const { data } = await request.get('/user/info/' + username + '/' + avatar)
  return data
}
