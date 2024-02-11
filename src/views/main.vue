<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'desc')">
      <h4>
        You can view and handle all hight level and emergent level todos on this page
      </h4>
    </div>
    <div :class="buildWrap(component, 'detail')">
      <Timeline :datas="todos"></Timeline>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";
import { AvatarMap, TeamAvatars, build, buildView, buildWrap, useTeam } from "../core";
import { Timeline } from "./plan";
import { todos } from "./todos";
const component = "Main";
</script>

<style lang="scss" scoped>
@import "../styles/views/main.scss";
</style>
