import { Avatars } from './avatar'
import { TodoBox } from './todo'
import { Team } from './team'
import { Option } from './common'

type UserLoginForm = {
  username: string
  password: string
}

type User = {
  username: string
  name: string
  // #[serde(skip_serializing)]
  // password: String
  avatar: Avatars
  email: string
  teamNumber: number
  todoNumber: number
  totalTodo: number
  todos: TodoBox
  teams: Option<Array<Team>>
  sendEmail: boolean
  sendMsg: boolean
}

type UserInfoChangeForm = {
  name: string
  email: string
  sendEmail: boolean
  sendMsg: boolean
}

export type { UserLoginForm, User, UserInfoChangeForm }
