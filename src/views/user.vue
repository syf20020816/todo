// 用户个人中心页
<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'avatar')">
      <img
        @click="avatarDrawer = true"
        id="userAvatar"
        :src="userStore.useAvatar(userStore.userInfo.avatar)"
        alt="Avatar"
      />
      <div :class="buildWrap('avatar', 'userInfo')">
        <div v-html="useSvg(SVGs.LOGO, 32)"></div>
        <div class="title">
          Hello!{{ userStore.userInfo.name }} Wishing you a pleasant day
        </div>
        <div class="email">Email:{{ userStore.userInfo.email }}</div>
      </div>
    </div>
    <div :class="buildWrap(component, 'details')">
      <div class="details-title">
        <span>UserInfo(ReadOnly)</span>
        <div class="info-extra">
          <el-rate
            v-model="busyValue"
            :icons="busyIcons"
            show-score
            disabled
            :void-icon="Coffee"
            :colors="['#409eff', '#FF9900', '#ff0000']"
            :score-template="busyTemplate"
          />
          <el-button type="primary">Solve TODOs</el-button>
        </div>
      </div>
      <div class="info-details-list">
        <div class="info-item" v-for="(item, index) in userInfoList" :key="index">
          <div class="item-title">{{ item.label }} :</div>
          <div class="item-value">{{ item.value }}</div>
        </div>
      </div>
      <el-progress
        :percentage="
          parseFloat(
            (1 - userStore.userInfo.todoNumber / userStore.userInfo.totalTodo).toFixed(2)
          ) * 100
        "
        :stroke-width="15"
        striped
        striped-flow
        :duration="100"
        :format="(percentage:any) => (percentage === 100 ? 'Full' : `${percentage}%`)"
      />
    </div>
    <div :class="buildWrap(component, 'teams')">
      <div style="display: flex">
        <div class="team-item" v-for="(item, index) in teamList" :key="index">
          <div class="header">
            <img :src="useTeam(item.teamIcon)" alt="" class="teamIcons" />
            <div class="info">
              <div>Team : {{ item.name }}</div>
              <div>Role : {{ item.role }}</div>
            </div>
          </div>
          <div style="font-size: 14px; width: 100%">
            {{ item.description }}
          </div>
          <el-button type="info">GOTO</el-button>
        </div>
      </div>
    </div>
  </div>
  <el-drawer
    v-model="avatarDrawer"
    title="Please choose your Avatar"
    :with-header="true"
    custom-class="avatarDrawer"
    size="400px"
  >
    <div class="drawer-details">
      <div class="avatar-list">
        <div
          @click="chooseAvatar(item)"
          class="avatar-item"
          v-for="(item, index) in avatarList"
          :key="index"
        >
          <img class="common-img" :src="userStore.useAvatar(item.label)" alt="Avatar" />
          <div class="infos">
            <div>avatar icon : {{ item.label }}</div>
          </div>
        </div>
      </div>
      <div class="chooseInfo">
        <div>Your Avatar : {{ userStore.userInfo.avatar }}</div>
        <div>Chosen : {{ chosenAvatar }}</div>
      </div>
      <div style="width: 100%">
        <el-button type="warning" @click="changeAvatar">
          <strong>change avatar</strong>
        </el-button>
      </div>
    </div>
  </el-drawer>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { Coffee, Platform, WarningFilled } from "@element-plus/icons-vue";
import { AvatarMap, Avatars, Teams, build, buildView, buildWrap, useTeam } from "../core";
import { user } from "../store/src/user";
import { SVGs, useSvg } from "../components";
const avatarDrawer = ref(false);
const component = "User";

const userStore = user();
const chosenAvatar = ref(userStore.userInfo.avatar);
const busyValue = ref(3);
const busyIcons = [Coffee, Platform, WarningFilled];

/**头像列表 */
const avatarList = computed(() => {
  let avatars: { label: Avatars; value: any }[] = [];
  AvatarMap.forEach((value, key, _map) => {
    avatars.push({
      label: key,
      value,
    });
  });
  return avatars;
});

/**选择头像事件 */
const chooseAvatar = (item: { label: Avatars; value: any }) => {
  chosenAvatar.value = item.label;
};

/**修改用户头像 */
const changeAvatar = () => {
  userStore.userInfo.avatar = chosenAvatar.value;
};

/**计算用户繁忙程度 */
const busyTemplate = computed(() => {
  let msg = "Free";
  if (busyValue.value >= 4) {
    msg = "Busy";
  } else if (busyValue.value >= 2) {
    msg = "Moderate";
  }
  return `busy level: ${msg}!`;
});

/**用户信息列表 */
const userInfoList = computed(() => {
  let { userInfo } = userStore;
  return [
    {
      label: "UserName",
      value: userInfo.username,
    },
    {
      label: "Name",
      value: userInfo.name,
    },
    {
      label: "Email",
      value: userInfo.email,
    },
    {
      label: "Avatar",
      value: userInfo.avatar,
    },
    {
      label: "Teams",
      value: userInfo.teamNumber,
    },
    {
      label: "Todos",
      value: userInfo.todoNumber,
    },
    {
      label: "TODO-fatal",
      value: userInfo.todos.fatal.length,
    },
    {
      label: "TODO-mid",
      value: userInfo.todos.mid.length,
    },
    {
      label: "TODO-low",
      value: userInfo.todos.low.length,
    },
    {
      label: "TODO-Focus",
      value: userInfo.todos.focus.length,
    },
  ];
});

const teamList = reactive<
  {
    id: string;
    name: string;
    role: string;
    teamIcon: Teams;
    description: string;
  }[]
>([
  {
    id: "0",
    name: "Surrealism",
    role: "Manager",
    teamIcon: Teams.Team1,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "1",
    name: "SurrealismUI",
    role: "Manager",
    teamIcon: Teams.Team2,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "2",
    name: "Slimk",
    role: "Watcher",
    teamIcon: Teams.Team3,
    description: "Surrealism is a SQL Builder",
  },
]);
</script>

<style lang="scss">
@import "../styles/views/user.scss";
</style>
