import { Request } from '../axios/index'
import { User, UserLoginForm, UserInfoChangeForm, Avatars } from '../../core'
import type { ApiResponse } from './type'

const request = Request.init()

export const createTeam = async (username: string, name: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get(`/team/${username}/${name}`)
  return data
}
