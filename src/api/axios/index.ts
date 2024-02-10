/**
 * ====================================
 *                ReadMe
 * 1. axios的配置文件
 * 2. 需要导出axios至main.ts注册（非模块化）
 * 3. 推荐在模块中直接引入调用init方法编写
 * ====================================
 */

//导入axios
import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import { ElMessage } from 'element-plus'

export class Request {
  public static axiosInstance: AxiosInstance

  public static init() {
    this.axiosInstance = axios.create({
      baseURL: 'http://localhost:10016/api/v1',
      timeout: 3600,
      headers: {
        // CORS 必填。可以填请求时Origin字段的值，表示允许本次跨域请求，也可以填“*”，表示允许任意地址的请求
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Credentials': 'true'
      }
    })
    this.axiosInstance.defaults.headers.common
    this.initInterceptors()
    // this.cors()
    return this.axiosInstance
  }

  public static initInterceptors() {
    this.axiosInstance.interceptors.request.use(
      config => {
        const token = window.localStorage.getItem('todo-token')
        if (token !== null && config?.headers) {
          console.log(token)
          //后续采用将token作为请求头中的参数进行向后端请求
          config.headers.token = token
        }
        return config
      },
      (error: any) => {
        console.log(error)
      }
    )

    this.axiosInstance.interceptors.response.use(
      (response: AxiosResponse) => {
        if (response.status === 200) {
          if (response.data['code'] === 200) {
            return response.data.data
          } else {
            this.errorHandle(response.data.msg)
            return undefined
          }
        } else {
          this.errorHandle(JSON.stringify(response))
          return null
        }
      },
      (error: any) => {
        this.errorHandle(error)
      }
    )
  }

  public static errorHandle(res: string) {
    ElMessage({
      type: 'error',
      message: res
    })
  }
}
