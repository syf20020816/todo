<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'desc')">
      <h4>
        <span style="margin: 0 16px"
          >You can view and handle all hight level and emergent level todos on this
          page</span
        >
        <el-switch v-model="todoType" active-text="Focus" inactive-text="Fatal" />
      </h4>
    </div>
    <div :class="buildWrap(component, 'detail')">
      <Timeline :datas="todos"></Timeline>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { reactive, computed, ref } from "vue";
import { AvatarMap, TeamAvatars, build, buildView, buildWrap, useTeam } from "../core";
import { Timeline } from "./plan";
import { user as userPinia } from "../store/src/user";
const component = "Main";
const userStore = userPinia();
const todoType = ref(true);
const todos = computed(() => {
  let { focus, fatal } = userStore.user.todos;

  if (todoType.value) {
    return focus;
  }
  return fatal;
});
</script>

<style lang="scss" scoped>
@import "../styles/views/main.scss";
</style>
