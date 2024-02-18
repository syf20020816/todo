<template>
  <div id="notice_container">
    <header>
      <h4>{{ data.title }}</h4>
    </header>
    <footer>
      <div>from: {{ data.notifier }}</div>
      <div>date: {{ data.date }}</div>
      <div class="check" @click="checkNotice">check</div>
    </footer>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
import { Notice } from "../../core";
import { ElMessage, ElMessageBox } from "element-plus";
const props = defineProps<{
  data: Notice;
}>();

const checkNotice = () => {
  let type = 0;
  let cancelButtonText =
    props.data.type === "todo"
      ? ((): string => {
          if (props.data.data.reviewers?.length !== 0) {
            type = 1;
            return "Send Notifications to Reviewers";
          }
          type = 2;
          return "Change Date Self";
        })()
      : "Cancel";

  ElMessageBox.confirm(props.data.description, "Handle", {
    confirmButtonText: "I have been completed",
    cancelButtonText,
    type: "warning",
  })
    .then(() => {
      ElMessage({
        type: "success",
        message: "Handled",
      });
    })
    .catch(() => {
      let message = "You can go to plan and delay TODO date or pending this TODO";
      if (type === 1) {
        message = "System has been sent notification to your reviewers";
      }

      ElMessage({
        type: "info",
        message,
      });
    });
};
</script>

<style lang="scss" scoped>
#notice_container {
  border-bottom: 2px dashed #444c58;
  padding-bottom: 6px;

  h4 {
    margin: 6px 0;
  }

  .check {
    text-align: end;
    color: #de511a;
    font-weight: 700;
    font-style: oblique;
    cursor: pointer;
    float: right;
    transform: translateY(-16px);
  }
}
</style>
