import Team1SVG  from "../../assets/team/team1.svg";
import Team2SVG  from "../../assets/team/team2.svg";
import Team3SVG  from "../../assets/team/team3.svg";
import Team4SVG  from "../../assets/team/team4.svg";

/**
 * 头像枚举
 */
export enum Teams {
  Team1='Team1',
  Team2='Team2',
  Team3="Team3",
  Team4="Team4"
}

/**
 * 头像枚举映射
 */
export const TeamMap = new Map<Teams,any>([
  [Teams.Team1 , Team1SVG],
  [Teams.Team2 , Team2SVG],
  [Teams.Team3 , Team3SVG],
  [Teams.Team4 , Team4SVG],
])

/**
 * 使用头像函数
 * 通过枚举得到映射结果
 * @param teamEnum Teams
 * @returns 
 */
export const useTeam = (teamEnum:Teams)=>{
  return TeamMap.get(teamEnum)
}