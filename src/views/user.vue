// 用户个人中心页
<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'avatar')">
      <img
        @click="avatarDrawer = true"
        id="userAvatar"
        :src="userStore.useAvatar(userStore.user.avatar)"
        alt="Avatar"
      />
      <div :class="buildWrap('avatar', 'userInfo')">
        <div v-html="useSvg(SVGs.LOGO, 32)"></div>
        <div class="title">
          Hello!{{ userStore.user.name }} Wishing you a pleasant day
        </div>
        <div class="email">Email:{{ userStore.user.email }}</div>
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
        :percentage="countSolve"
        :stroke-width="15"
        striped
        striped-flow
        :duration="100"
        :format="(percentage:any) => (percentage === 100 ? 'Full' : `${percentage}%`)"
      />
    </div>
    <div :class="buildWrap(component, 'teams')">
      <div style="display: flex" v-if="hasTeam">
        <div class="team-item" v-for="(item, index) in userStore.user.teams" :key="index">
          <div class="header">
            <img :src="useTeam(item.avatar)" alt="" class="teamIcons" />
            <div class="info">
              <div>Team : {{ item.name }}</div>
              <div>Role : {{ setTeamRole(item.owner) }}</div>
            </div>
          </div>
          <div style="font-size: 14px; width: 100%">
            {{ item.description }}
          </div>
          <el-button type="info">GOTO</el-button>
        </div>
      </div>
      <div v-else class="team-no">
        <p>
          You now have no Team.
          <a @click="toCollaborate" class="create-wrapper">Click here to create!</a>
        </p>
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
        <div>Your Avatar : {{ userStore.user.avatar }}</div>
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
import {
  AvatarMap,
  Avatars,
  TeamAvatars,
  User,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import { user } from "../store/src/user";
import { SVGs, useSvg } from "../components";
import { useRouter } from "vue-router";

const avatarDrawer = ref(false);
const component = "User";
const router = useRouter();
const userStore = user();
const chosenAvatar = ref(userStore.user.avatar);
const busyValue = computed(() => {
  let { todos } = userStore.user;
  let { fatal, focus } = todos ?? {
    fatal: [],
    focus: [],
  };
  return fatal.length + focus.length;
});
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
  userStore.user.avatar = chosenAvatar.value;
};

/**计算用户繁忙程度 */
const busyTemplate = computed(() => {
  let { todoNumber } = userStore.user;
  let busy = busyValue.value / todoNumber;
  let msg = "Free";
  if (busy >= 0.6) {
    msg = "Busy";
  } else if (busyValue.value >= 0.4) {
    msg = "Moderate";
  }
  return `busy level: ${msg}!`;
});

/**用户信息列表 */
const userInfoList = computed(() => {
  let { user } = userStore;
  return [
    {
      label: "UserName",
      value: user.username,
    },
    {
      label: "Name",
      value: user.name,
    },
    {
      label: "Email",
      value: user.email,
    },
    {
      label: "Avatar",
      value: user.avatar,
    },
    {
      label: "TeamAvatars",
      value: user.teamNumber,
    },
    {
      label: "Todos",
      value: user.todoNumber,
    },
    {
      label: "TODO-fatal",
      value: user.todos?.fatal.length ?? 0,
    },
    {
      label: "TODO-mid",
      value: user.todos?.mid.length ?? 0,
    },
    {
      label: "TODO-low",
      value: user.todos?.low.length ?? 0,
    },
    {
      label: "TODO-Focus",
      value: user.todos?.focus.length ?? 0,
    },
  ];
});

const teamList = reactive<
  {
    id: string;
    name: string;
    role: string;
    teamIcon: TeamAvatars;
    description: string;
  }[]
>([
  {
    id: "0",
    name: "Surrealism",
    role: "Manager",
    teamIcon: TeamAvatars.Team1,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "1",
    name: "SurrealismUI",
    role: "Manager",
    teamIcon: TeamAvatars.Team2,
    description: "Surrealism is a SQL Builder",
  },
  {
    id: "2",
    name: "Slimk",
    role: "Watcher",
    teamIcon: TeamAvatars.Team3,
    description: "Surrealism is a SQL Builder",
  },
]);

const countSolve = computed(() => {
  let { todoNumber, totalTodo } = userStore.user;
  if (todoNumber === 0 || totalTodo === 0) {
    return 100;
  }
  let solve = (1 - todoNumber / todoNumber).toFixed(2);
  console.log(todoNumber / totalTodo);
  let solveF = parseFloat(solve) * 100;
  return solveF;
});

const setTeamRole = computed(() => (owner: User) => {
  let { username: ownername } = owner;
  let { username } = userStore.user;
  if (username === ownername) {
    return "Manager";
  } else {
    return "Parter";
  }
});

const hasTeam = computed(() => {
  return userStore.user.teams?.length ?? 0 !== 0;
});

const toCollaborate = () => {
  router.push({ path: "collaborate" });
};
</script>

<style lang="scss">
@import "../styles/views/user.scss";
</style>
