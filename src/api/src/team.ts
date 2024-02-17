import { Request } from '../axios/index'
import { User, UserLoginForm, UserInfoChangeForm, Avatars, Team } from '../../core'
import type { ApiResponse } from './type'

const request = Request.init()

export const createTeam = async (username: string, name: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get(`/team/${username}/${name}`)
  return data
}

export const updateTeamMember = async (membername: string, team: Team): Promise<boolean> => {
  const { data } = await request.put(`/team/${membername}`, team)
  return data
}

export const updateTeamInfo = async (team: Team): Promise<boolean> => {
  const { data } = await request.put(`/team`, team)
  return data
}
