<template>
  <div :id="buildView(component)">
    <el-tabs class="plan-tabs">
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.CREATE_PREVIEW, 16)"></div>
            <span class="title">Create and Preview</span>
          </span>
        </template>
        <div class="c_p_wrapper">
          <CreatePreview :datas="todos"></CreatePreview>
        </div>
      </el-tab-pane>
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.CALENDER, 16)"></div>
            <span class="title">Calendar</span>
          </span>
        </template>
        <div class="calendar_wrapper">
          <Calendar :datas="todos"></Calendar>
        </div>
      </el-tab-pane>
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.TIMELINE, 16)"></div>
            <span class="title">Timeline</span>
          </span>
        </template>
        <div class="timeline_wrapper">
          <Timeline :datas="todos"></Timeline>
        </div>
      </el-tab-pane>
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.TABLE, 16)"></div>
            <span class="title">Table</span>
          </span>
        </template>
        <div class="table_wrapper">
          <Table :datas="todos"></Table>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import { Avatars, buildView, PriorityEnum, Status, ITODO } from "../../core";
import { SVGs, useSvg } from "../../components";
import { CreatePreview, Calendar, Timeline, Table } from "./index";
const component = "Plan";
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

<style lang="scss">
@import "../../styles/views/plan.scss";
</style>
