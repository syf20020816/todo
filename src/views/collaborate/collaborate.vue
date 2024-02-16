<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'teams')">
      <div v-if="teamList?.length" style="height: 100%; width: 100%">
        <div v-for="(item, index) in teamList" :key="index" class="team_wrapper">
          <div class="team-avatar">
            <img :src="useTeam(item.avatar)" alt="" class="teamIcons" />
          </div>
          <div class="team-details">
            <h5 class="name">{{ item.name }}</h5>
            <p class="desc">{{ item.description }}</p>
          </div>
        </div>
      </div>
      <div v-else>
        <h4>😅You currently do not have a team, please create it first</h4>
        <div class="add-wrapper" @click="createNewTeam">
          <div v-html="useSvg(SVGs.MENU_COLLABORATE, 32)"></div>
        </div>
      </div>
    </div>
    <div :class="buildWrap(component, 'details')">
      <div :class="buildWrap('details', 'team-member')">
        <div
          class="team-member-wrapper"
          v-for="item in memberList"
          :key="item.id"
          @click="chooseMember(item)"
        >
          <div class="team-avatar">
            <img :src="useAvatar(item.avatar)" alt="" class="teamIcons" />
          </div>
          <div class="team-details">
            <h5 class="name">{{ item.name }}</h5>
            <p class="desc">{{ item.email }}</p>
          </div>
        </div>
      </div>
      <div :class="buildWrap('details', 'panel')">
        <Panel :member="currentMember" @create="createNewTeam"></Panel>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import Panel from "./components/panel.vue";
import {
  AvatarMap,
  Avatars,
  Todo,
  Priorities,
  Status,
  TeamAvatars,
  build,
  buildView,
  buildWrap,
  useAvatar,
  useTeam,
} from "../../core";
import api from "../../api";
import { user as userPinia } from "../../store/src/user";
import { SVGs, useSvg } from "../../components";
import { ElMessage, ElMessageBox } from "element-plus";
const component = "Collaborate";
const userStore = userPinia();

const teamList = computed(() => {
  let { teams } = userStore.user;
  return teams;
});

const memberList = reactive<
  {
    id: string;
    name: string;
    role: string;
    avatar: Avatars;
    email: string;
  }[]
>([
  {
    id: "0",
    name: "Surrealism",
    role: "Manager",
    avatar: Avatars.Miner,
    email: "Surrealism is a SQL Builder",
  },
  {
    id: "1",
    name: "SurrealismUI",
    role: "Manager",
    avatar: Avatars.Adventurer,
    email: "Surrealism is a SQL Builder",
  },
]);

const currentMember = ref<{
  id: string;
  name: string;
  role: string;
  avatar: Avatars;
  email: string;
}>(memberList[0]);

const chooseMember = (item: any) => {
  currentMember.value = item;
};
const createNewTeam = () => {
  ElMessageBox.prompt("create a new team for self", "Create Team", {
    confirmButtonText: "Create",
    cancelButtonText: "Cancel",
    inputPlaceholder: "Please enter a new team name",
  }).then(({ value }) => {
    let name = value.trim();
    if (name.length === 0) {
      ElMessage({
        type: "warning",
        message: "Please do not enter an empty team name",
      });
      return;
    }
    api.team.createTeam(userStore.user.username, name).then((user) => {
      console.log(user);
      if (typeof user !== "undefined") {
        userStore.setUser(user);
      }
      ElMessage({
        type: "success",
        message: "Team created successfully",
      });
    });
  });
};
</script>

<style lang="scss" scoped>
@import "../../styles/views/collaborate.scss";
</style>
