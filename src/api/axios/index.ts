/**
 * ====================================
 *                ReadMe
 * 1. axios的配置文件
 * 2. 需要导出axios至main.ts注册（非模块化）
 * 3. 推荐在模块中直接引入调用init方法编写
 * ====================================
 */

//导入axios
import axios, { AxiosInstance,AxiosRequestConfig,AxiosResponse } from 'axios'

export class Request{
  public static axiosInstance:AxiosInstance

  public static init(){
    this.axiosInstance = axios.create({
      baseURL:'http://localhost:8484/',
      timeout:1500
    })
    this.initInterceptors()
    return this.axiosInstance
  }

  public static initInterceptors(){
    this.axiosInstance.interceptors.request.use(
      (config:AxiosRequestConfig)=>{
        const token = window.localStorage.getItem('token')
        if(token!=null&&config?.headers){
          console.log(token)
          //后续采用将token作为请求头中的参数进行向后端请求
           config.headers.token  = "test"
           
        }
        return config
      },
      (error:any)=>{
        console.log(error)
      }
    )

    this.axiosInstance.interceptors.response.use(
      (response:AxiosResponse)=>{
        if(response.status===200){
          return response.data
        }else{
          this.errorHandle(response)
          return null
        }
      },
      (error:any)=>{
        this.errorHandle(error)
      }
    )
  }


  public static errorHandle(res:any){
    console.log(res)
  }
}
