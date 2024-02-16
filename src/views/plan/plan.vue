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
          <CreatePreview :datas="userStore.todos"></CreatePreview>
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
          <Calendar :datas="userStore.todos"></Calendar>
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
          <Timeline :datas="userStore.todos"></Timeline>
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
          <Table :datas="userStore.todos"></Table>
        </div>
      </el-tab-pane>
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <div class="svg" v-html="useSvg(SVGs.PENDING, 16)"></div>
            <span class="title">Pending</span>
          </span>
        </template>
        <div class="table_wrapper">
          <Table :datas="userStore.todos"></Table>
        </div>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from 'vue'
import { Avatars, buildView, Priorities, Status, Todo } from '../../core'
import { SVGs, useSvg } from '../../components'
import { CreatePreview, Calendar, Timeline, Table } from './index'
import { user as userPinia } from '../../store/src/user'
const component = 'Plan'
const userStore = userPinia()
</script>

<style lang="scss">
@import '../../styles/views/plan.scss';
</style>
