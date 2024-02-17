<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'teams')">
      <div v-if="userStore.user.teams?.length" style="height: 100%; width: 100%">
        <div
          v-for="(item, index) in userStore.user.teams"
          :key="index"
          class="team_wrapper"
          @click="currentTeam = item"
        >
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
        <div v-if="currentTeam" style="height: 100%; width: 100%">
          <div
            class="team-member-wrapper"
            v-for="item in currentTeam?.members"
            :key="item.username"
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
        <div v-else>
          <h4>You can choose a team and get team members</h4>
        </div>
      </div>
      <div :class="buildWrap('details', 'panel')">
        <Panel
          :data="currentTeam"
          @create="createNewTeam"
          @add="addMember"
          :add-member-disabled="addMemberDisabled"
        ></Panel>
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
  Team,
} from "../../core";
import api from "../../api";
import { user as userPinia } from "../../store/src/user";
import { SVGs, useSvg } from "../../components";
import { ElMessage, ElMessageBox } from "element-plus";
const component = "Collaborate";
const userStore = userPinia();

const currentTeam = ref<Team>();
const currentMember = ref();

const addMemberDisabled = computed(() => {
  return typeof currentTeam.value === "undefined";
});

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

const addMember = () => {
  ElMessageBox.prompt("add new team member", "Add Member", {
    confirmButtonText: "Add",
    cancelButtonText: "Cancel",
    inputPlaceholder: "Please enter member's username",
  }).then(({ value }) => {
    let name = value.trim();
    if (name.length === 0) {
      ElMessage({
        type: "warning",
        message: "Please do not enter an empty member username",
      });
      return;
    }

    let members = currentTeam.value?.members;

    if (members?.filter((member) => member.username === name).length !== 0) {
      ElMessage({
        type: "warning",
        message: "The current user already exists",
      });
      return;
    }

    api.team
      .updateTeamMember(name, currentTeam.value!)
      .then((update) => {
        if (update) {
          api.user.getUserInfo(userStore.user.username).then((user) => {
            userStore.setUser(user!);
            currentTeam.value = undefined;
          });
          ElMessage({
            type: "success",
            message: "Add member successfully",
          });
        } else {
          ElMessage({
            type: "error",
            message: "Add member failed, please check member's username",
          });
        }
      })
      .catch(() => {});
  });
};
</script>

<style lang="scss" scoped>
@import "../../styles/views/collaborate.scss";
</style>
