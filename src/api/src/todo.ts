/**
 * ========================================
 *                  ReadMe
 * 1. Axios模块化示例，我们需要导入我们写的axios配置文件
 * 2. 我们需要调用init方法进行初始化
 * 3. 使用其中的get,post...方法进行请求即可
 * 4. 编写完毕后导出函数
 * 5. 在api.ts中引入进行统一管理
 * ========================================
 */
import { Status, Todo, User } from '../../core'
import { Request } from '../axios/index'
import { ApiResponse } from './type'

const request = Request.init()

export const addNewTodo = async (todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.post('/todo/create', todo)
  return data
}

export const deleteTodo = async (username: string, id: string): Promise<ApiResponse<User>> => {
  const { data } = await request.delete('/todo/' + username + '/' + id)
  return data
}

export const updateTodo = async (username: string, id: string, todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.put('/todo/' + username + '/' + id, todo)
  return data
}

export const failedTodo = async (username: string, id: string, todo: Todo): Promise<ApiResponse<User>> => {
  const { data } = await request.put('/todo/failed/' + username + '/' + id, todo)
  return data
}

export const updateTodoStatus = async (id: String, status: Status): Promise<boolean> => {
  const { data } = await request.get('/todo/' + id + '/' + status)
  return data
}

export const completedTodo = async (username: string, todoId: string): Promise<ApiResponse<User>> => {
  const { data } = await request.get('/todo/complete/' + username + '/' + todoId)
  return data
}
