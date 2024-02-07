<template>
  <div :id="buildView(component)">
    <div :class="buildWrap(component, 'left')">
      <el-calendar>
        <template #date-cell="{ data }">
          <p :class="data.isSelected ? 'is-selected' : ''" @click="getTODO(data)">
            {{ data.day.split("-").slice(1).join("-") }}
            {{ data.isSelected ? "🚩" : "" }}
          </p>
        </template>
      </el-calendar>
    </div>
    <div :class="buildWrap(component, 'mid')">
      <el-timeline>
        <el-timeline-item
          v-for="(activity, index) in activities"
          :key="index"
          :timestamp="activity.timestamp"
        >
          <h4>{{ activity.content }}</h4>
          <el-button type="primary">📃Check</el-button>
        </el-timeline-item>
      </el-timeline>
    </div>
    <div :class="buildWrap(component, 'right')">
      <TODOItem :current-todo="currentItem"></TODOItem>
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
import { TODOItem } from "./plan";

const component = "History";

const getTODO = (data: any) => {
  console.log(data.day);
};

const currentItem = ref<ITODO>();

const activities = [
  {
    content: "Event start",
    timestamp: "2018-04-15",
  },
  {
    content: "Approved",
    timestamp: "2018-04-13",
  },
  {
    content: "Success",
    timestamp: "2018-04-11",
  },
];
</script>

<style lang="scss" scoped>
@import "../styles/views/history.scss";
</style>
