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
import { Request } from "../axios/index";

const request = Request.init();

/**
 * axios的get方法这里更加具体将每个地方的类型都进行了规范
 * @param params
 * @returns
 */
export const getUserInfo = async (params: string): Promise<any> => {
  const { data: res } = await request.get("/user/info/" + params);
  return res;
};

/**
 * axios的post方法，这里我演示简单写一下函数不声明类型
 * 则默认为Promise<any>,userLogin的返回值类型则是any
 * @param params
 * @returns
 */
export const userLogin = async (params: any) => {
  const { data: res } = await request.post("/user/login", params);
  return res;
};
