/**
 * ==============================
 *            ReadMe
 * 1. 作为Api的出口文件
 * 2. 若使用此方式（axios-api模块化）请注释掉main.js中案例提供的默认$http的全局axios
 * ==============================
 */
import { userLogin, getUserInfo } from './src/defaultApi'
import { signin, signup } from './src/user'

export default {
  user: {
    signin,
    signup
  }
}
