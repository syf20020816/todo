<template>
  <div id="team-panel-wrapper">
    <div class="panel-member-detail-wrapper"></div>
    <div class="panel-team-detail-wrapper"></div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive } from "vue";
</script>

<style lang="scss" scoped>
@use "../../../styles/src/var.scss" as *;

#team-panel-wrapper {
  width: 100%;
  height: 100%;
  border-radius: 10px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid $bg-color-hover;
}
</style>
