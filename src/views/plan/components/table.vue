<template>
  <div id="table-view">
    <el-table :data="datas">
      <el-table-column type="index" label="Index" width="100px" fixed="left" />
      <el-table-column label="Name" prop="name" width="200px" />
      <el-table-column label="Priority" prop="priority" width="200px" />
      <el-table-column label="Reviewers" width="200px" prop="reviewers">
        <template #default="{ row, $index }">
          <div>{{ getReviewerName(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Performers" prop="performers" width="200px">
        <template #default="{ row, $index }">
          <div>{{ getPerformersName(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Date" prop="date" width="360px">
        <template #default="{ row, $index }">
          <div>{{ getDate(row, $index) }}</div>
        </template>
      </el-table-column>
      <el-table-column label="Tags" prop="tags" width="240px">
        <template #default="{ row, $index }">
          <span>
            <el-tag
              style="margin: 0 4px"
              v-for="tag in getTags(row, $index)"
              :key="tag.label"
              :type="tag.type"
              size="small"
              class="mx-tag"
              round
              :effect="tag.effect"
              >{{ tag.label }}</el-tag
            >
          </span>
        </template>
      </el-table-column>
      <el-table-column align="right" width="120px" fixed="right">
        <template #header>
          <span>Operation</span>
        </template>
      </el-table-column>
      <el-table-column type="expand" fixed="right">
        <template #default="props">
          <div class="expand-table-wrapper">
            <TODOItem :current-todo="props.row" :is-change="false"></TODOItem>
          </div>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from "vue";
import { Todo } from "../../../core";
import { TODOItem } from "../index";
const props = defineProps<{
  datas: Todo[];
}>();

const getReviewerName = computed(() => (row: any, index: number) => {
  return row.reviewers[index]?.name ?? "";
});

const getPerformersName = computed(() => (row: any, index: number) => {
  return row.performers[index]?.name ?? "";
});

const getDate = computed(() => (row: any, _index: number) => {
  let { start, end } = row.date ?? {
    start: "",
    end: "",
  };
  return `${start ?? ""} ~ ${end ?? ""}`;
});

const getTags = computed(() => (row: any, _index: number): {
  type: string;
  effect: string;
  label: string;
}[] => {
  return row.tags;
});
</script>

<style lang="scss">
@use '../../../styles/src/var.scss' as *;

#table-view {
  box-sizing: border-box;
  height: 100%;
  // padding: 6px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  overflow-y: scroll;
  scrollbar-width: thin;
  flex-wrap: wrap;

  // 覆盖el-table默认颜色
  // 注意：这里覆盖了element-plus的样式，如果需要使用element-plus的样式，需要将element-plus的样式覆盖
  // 例如：@import "element-plus/theme-chalk/src/common/var.scss";
  // 或者使用!important来覆盖样式
  .el-table {
    --el-table-bg-color: #3b3a39;
    --el-table-header-bg-color: #252423;
    --el-table-tr-bg-color: #3b3a39;
    --el-table-row-hover-bg-color: #252423;
    --el-table-border: 1px solid #3b3a39;
    --el-table-border-color: #3b3a39;
    --el-table-text-color: #fff;
    --el-table-header-text-color: #bfbfbf;
    --el-table-expanded-cell-bg-color: #252423;

    .el-table__body-wrapper tr td.el-table-fixed-column--left {
      background: #3b3a39;
    }

    .el-table__body-wrapper tr td.el-table-fixed-column--right {
      background: #3b3a39;
    }
  }

  .expand-table-wrapper {
    width: calc(100% - 240px);
    display: flex;
    min-width: 620px;
    align-items: center;
    box-sizing: border-box;
    padding: 16px;
    justify-content: center;
  }
}
</style>
