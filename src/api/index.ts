/**
 * ==============================
 *            ReadMe
 * 1. 作为Api的出口文件
 * 2. 若使用此方式（axios-api模块化）请注释掉main.js中案例提供的默认$http的全局axios
 * ==============================
 */
import { addNewTodo, deleteTodo, updateTodo, updateTodoStatus, completedTodo, failedTodo } from './src/todo'
import { signin, signup, getUserInfo, setUserInfo, setUserAvatar } from './src/user'
import { createTeam, updateTeamMember, updateTeamInfo, createTeamTodo } from './src/team'
export default {
  user: {
    signin,
    signup,
    getUserInfo,
    setUserInfo,
    setUserAvatar
  },
  todo: {
    addNewTodo,
    deleteTodo,
    updateTodo,
    updateTodoStatus,
    completedTodo,
    failedTodo
  },
  team: {
    createTeam,
    updateTeamMember,
    updateTeamInfo,
    createTeamTodo
  }
}
