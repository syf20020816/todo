import Avatar1Png from '../../assets/avatar/avatar1.png';
import Avatar2Png from '../../assets/avatar/avatar2.png';
import Avatar3Png from '../../assets/avatar/avatar3.png';
/**
 * 头像枚举
 */
export enum Avatars {
  Avatar1 = 'Worker',
  Avatar2 = "Miner",
  Avatar3 = "Adventurer"
}

/**
 * 头像枚举映射
 */
export const AvatarMap = new Map<Avatars,any>([
  [Avatars.Avatar1 , Avatar1Png],
  [Avatars.Avatar2 , Avatar2Png],
  [Avatars.Avatar3 , Avatar3Png]
])

/**
 * 使用头像函数
 * 通过枚举得到映射结果
 * @param avatarEnum 
 * @returns 
 */
export const useAvatar = (avatarEnum:Avatars)=>{
  return AvatarMap.get(avatarEnum)
}