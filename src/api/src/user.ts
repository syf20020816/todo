/**
 * 用户相关API
 * - 登录
 * - 注册
 * - 获取用户信息
 * - 等
 */
import { Request } from '../axios/index'
import { User, UserLoginForm } from '../../core'
import type { ApiResponse } from './type'

const request = Request.init()

/**
 *
 * @param params
 * @returns
 */
export const signin = async (params: UserLoginForm): Promise<any> => {
  const { data: res } = await request.post('/user/signin/', params)
  return res
}

export const signup = async (params: UserLoginForm): Promise<ApiResponse<User>> => {
  const { data: res } = await request.post('/user/signup/', params)
  return res
}

/**
 * axios的post方法，这里我演示简单写一下函数不声明类型
 * 则默认为Promise<any>,userLogin的返回值类型则是any
 * @param params
 * @returns
 */
export const userLogin = async (params: any) => {
  const { data: res } = await request.post('/user/login', params)
  return res
}
