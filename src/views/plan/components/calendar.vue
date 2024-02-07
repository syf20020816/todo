<template>
  <div id="calendar-view">
    <div class="left">
      <el-calendar>
        <template #date-cell="{ data }">
          <p :class="data.isSelected ? 'is-selected' : ''" @click="getTODO(data)">
            {{ data.day.split("-").slice(1).join("-") }}
            {{ data.isSelected ? "🚩" : "" }}
          </p>
        </template>
      </el-calendar>
    </div>
    <div class="right">
      <el-collapse accordion>
        <el-collapse-item :name="item.id" v-for="item in props.datas" :key="item.id">
          <template #title>
            <div class="collapse-title-wrapper">
              <div>
                <span class="priority" :style="getPriorityDot(item)"></span>
                <span
                  class="priority"
                  style="border-radius: 50%"
                  :style="getStatusDot(item)"
                ></span>
                <span class="collapse-title">{{ item.name }}</span>
              </div>
              <el-button type="danger" size="small">Remove TODO</el-button>
            </div>
          </template>
          <div style="height: 360px">
            <TODOItem :current-todo="item"></TODOItem>
          </div>
        </el-collapse-item>
      </el-collapse>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { ITODO, PriorityEnum, usePriorityColor, Status, useStatus } from "../../../core";
import { TODOItem } from "../index";
const props = defineProps<{ datas: ITODO[] }>();
const emits = defineEmits(["getTODOs"]);

const getPriorityDot = computed(() => (item: ITODO) => {
  let { priority } = item || PriorityEnum.Low;
  return `background-color : ${usePriorityColor(priority)}`;
});

const getStatusDot = computed(() => (item: ITODO) => {
  let { status } = item || Status.NOT_START;
  return `background-color : ${useStatus(status)}`;
});

const getTODO = (data: any) => {
  emits("getTODOs", {
    date: data.day,
  });
};
</script>

<style lang="scss">
@use "../../../styles/src/var.scss" as *;
#calendar-view {
  height: 100%;
  width: 100%;
  box-sizing: border-box;
  padding: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  .left {
    height: 100%;
    box-sizing: border-box;
    padding: 0 6px;
    width: 50%;
    .el-calendar {
      background-color: $bg-color-hover;
    }
    .el-calendar-day {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .current {
      &:hover {
        background-color: $force-color !important;
      }
    }
    .is-today {
      color: $bg-color-dark !important;
    }
    :deep(.el-calendar-table td.is-selected) {
      background-color: $force-color !important;
    }
    .is-selected {
      // background-color: var(--el-calendar-selected-bg-color);
      background-color: $force-color;
    }
  }
  .right {
    height: 100%;
    width: 50%;
    padding: 0 6px;
    box-sizing: border-box;
    border-left: 1px dashed $bg-color-hover;
    .priority {
      display: inline-block;
      height: 12px;
      width: 12px;
      border-radius: 2px;
      margin: 0 4px;
    }
    .collapse-title-wrapper {
      box-sizing: border-box;
      padding: 0 8px;
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      .collapse-title {
        font-weight: 700;
        color: $bg-color-light;
      }
    }

    .el-collapse-item__header {
      background-color: $bg-color-menu !important;
    }
    .el-collapse-item__content {
      margin: 0;
      padding: 0;
      background-color: $bg-color-hover !important;
    }
  }
}
</style>
