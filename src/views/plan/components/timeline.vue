<template>
  <div id="timeline-view">
    <div class="left">
      <el-timeline>
        <el-timeline-item
          :timestamp="`${item.date.start}~${item.date.end}`"
          placement="top"
          v-for="(item, index) in datas"
          :key="index"
        >
          <el-card @click="currentTodo = item">
            <h4>
              <span class="priority" :style="getPriorityDot(item)"></span>
              <span
                class="priority"
                style="border-radius: 50%"
                :style="getStatusDot(item)"
              ></span>
              <span class="collapse-title">{{ item.name }}</span>
            </h4>
            <p>{{ item.description }}</p>
            <div class="operation-btn-wrapper">
              <el-tag
                style="margin: 0 6px"
                v-for="tag in item.tags"
                :key="tag.label"
                :type="tag.type"
                size="small"
                class="mx-tag"
                round
                :effect="tag.effect"
              >
                {{ tag.label }}
              </el-tag>
            </div>
          </el-card>
        </el-timeline-item>
      </el-timeline>
    </div>
    <div class="right">
      <TODOItem :current-todo="currentTodo" :is-change="false"></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { Todo, Priorities, usePriorityColor, Status, useStatus } from "../../../core";
import { TODOItem } from "../index";
const props = defineProps<{ datas: Todo[] }>();

const currentTodo = ref<Todo>();
const getPriorityDot = computed(() => (item: Todo) => {
  let { priority } = item || Priorities.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

const getStatusDot = computed(() => (item: Todo) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;
#timeline-view {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  .left {
    width: 50%;
    height: 100%;
    box-sizing: border-box;
    padding-right: 12px;
    overflow-y: scroll;
    scrollbar-width: thin;
    .el-card {
      cursor: pointer;
      border: none;
      color: $bg-color-light;
    }
    :deep(.el-card__body) {
      padding: 6px;
      background-color: $bg-color-hover;
      border: none;
      h4 {
        margin: 12px 0;
        .priority {
          display: inline-block;
          height: 12px;
          width: 12px;
          border-radius: 2px;
          margin: 0 4px;
        }
      }
    }
    .operation-btn-wrapper {
      box-sizing: border-box;
      padding-bottom: 16px;
    }
  }
  .right {
    width: 50%;
    box-sizing: border-box;
    padding: 0 6px;
    height: 100%;
    border-left: 1px dashed $bg-color-hover;
  }
}
</style>
