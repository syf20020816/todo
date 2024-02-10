import { Avatars } from './avatar'
import { TodoBox } from './todo'
import { Team } from './team'
import { Option } from './common'

type UserLoginForm = {
  username: string
  password: string
}

type User = {
  username: String
  name: String
  // #[serde(skip_serializing)]
  password: String
  avatar: Avatars
  email: String
  team_number: number
  todo_number: number
  total_todo: number
  todos: TodoBox
  teams: Option<Array<Team>>
}
export type { UserLoginForm, User }
