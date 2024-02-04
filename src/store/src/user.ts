import { defineStore } from "pinia";
import {Avatars,useAvatar} from "../../core"

// useStore 可以是 useUser、useCart 之类的任何东西
// 第一个参数是应用程序中 store 的唯一 id
export const user = defineStore("user", {
  // other options...
  state: () => {
    return {
      
      userInfo:{
        username:"Matt111",
        name : "Matt",
        avatar : Avatars.Avatar1,
        email:"syf20020816@outlook.com",
        teamNumber:3,
        todoNumber:12,
        totalTodo:23,
        todos:{
          low : [],
          mid:[],
          fatal:[],
          //关注
          focus:[]
        } 
      }
    };
  },
  actions: {
    useAvatar
  },
});
