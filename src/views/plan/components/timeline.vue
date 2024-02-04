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
            <div>
              <el-button type="danger">Remove TODO</el-button>
            </div>
          </el-card>
        </el-timeline-item>
      </el-timeline>
    </div>
    <div class="right">
      <TODOItem :current-todo="currentTodo"></TODOItem>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { ITODO, PriorityEnum, usePriorityColor, Status, useStatus } from "../../../core";
import { TODOItem } from "../index";
const props = defineProps<{ datas: ITODO[] }>();

const currentTodo = ref<ITODO>();
const getPriorityDot = computed(() => (item: ITODO) => {
  let { priority } = item || PriorityEnum.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

const getStatusDot = computed(() => (item: ITODO) => {
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
    width: 55%;
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
  }
  .right {
    width: 45%;
    box-sizing: border-box;
    padding: 0 6px;
    height: 100%;
    border-left: 1px dashed $bg-color-hover;
  }
}
</style>
