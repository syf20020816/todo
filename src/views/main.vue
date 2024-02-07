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
import { ref, reactive } from "vue";
import {
  AvatarMap,
  Avatars,
  ITODO,
  PriorityEnum,
  Status,
  Teams,
  build,
  buildView,
  buildWrap,
  useTeam,
} from "../core";
import { Timeline } from "./plan";
const component = "Main";

const todos = ref<ITODO[]>([
  {
    id: "001",
    name: "Fix card display",
    priority: PriorityEnum.Emergent,
    reviewers: [
      {
        id: "1",
        name: "Jany",
        email: "Jany@gmail.com",
        avatar: Avatars.Avatar1,
      },
    ],
    performers: [
      {
        id: "2",
        name: "Surrog",
        email: "Surrog@gmail.com",
        avatar: Avatars.Avatar2,
      },
    ],
    date: {
      start: "10.30",
      end: "14.30",
      during: 2,
    },
    tags: [
      {
        type: "info",
        effect: "dark",
        label: "Rust",
      },
      {
        type: "success",
        effect: "dark",
        label: "CSS",
      },
    ],
    status: Status.IN_PROGRESS,
    description: "test",
  },
  {
    id: "002",
    name: "Fix card display2",
    priority: PriorityEnum.Emergent,
    reviewers: [
      {
        id: "1",
        name: "Jany",
        email: "Jany@gmail.com",
        avatar: Avatars.Avatar1,
      },
    ],
    performers: [
      {
        id: "2",
        name: "Surrog",
        email: "Surrog@gmail.com",
        avatar: Avatars.Avatar2,
      },
    ],
    date: {
      start: "10.30",
      end: "14.30",
      during: 2,
    },
    tags: [
      {
        type: "info",
        effect: "dark",
        label: "Rust",
      },
      {
        type: "success",
        effect: "dark",
        label: "CSS",
      },
    ],
    status: Status.IN_PROGRESS,
    description: "test",
  },
]);
</script>

<style lang="scss" scoped>
@import "../styles/views/main.scss";
</style>
