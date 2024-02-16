<template>
  <div id="team-panel-wrapper">
    <div class="panel-tool-wrapper">
      <el-button type="primary" @click="createNewTeam">Create Team</el-button>
      <el-button type="primary">Add Member</el-button>
    </div>
    <div class="panel-member-detail-wrapper">
      <header class="header">
        <img :src="useAvatar(member!.avatar)" alt="" class="avatar" />
      </header>
      <center class="center"></center>
      <footer class="footer"></footer>
    </div>
    <div class="panel-team-detail-wrapper"></div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import { Avatars, useAvatar } from "../../../core";
const props = defineProps<{
  member: {
    id: string;
    name: string;
    role: string;
    avatar: Avatars;
    email: string;
  };
}>();

const emits = defineEmits(["create"]);

const createNewTeam = () => {
  emits("create");
};
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;

#team-panel-wrapper {
  width: 100%;
  height: 100%;
  border-radius: 10px;
  overflow: scroll;
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid $bg-color-hover;
  flex-wrap: wrap;
  align-content: space-between;
  .panel-tool-wrapper {
    height: 60px;
    width: 100%;
    padding: 6px;
    box-sizing: border-box;
    margin: 12px 16px;
    background-color: $bg-color-menu;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }
  .panel-member-detail-wrapper {
    height: calc(320px - 32px);
    width: 100%;
    background-color: $bg-color-hover;
    box-sizing: border-box;
    margin: 8px 16px;
    border-radius: 4px;
    .header {
      display: flex;
      align-items: center;
      justify-content: flex-start;
      height: 72px;
      padding: 6px;
      box-sizing: border-box;
      .avatar {
        height: 60px;
        width: 60px;
        border-radius: 50%;
      }
    }
  }
  .panel-team-detail-wrapper {
    height: calc(100% - 320px - 92px);
    width: 100%;
    margin: 8px 16px;
    background-color: $bg-color-hover;
    box-sizing: border-box;
  }
}
</style>
