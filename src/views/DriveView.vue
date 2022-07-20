<script setup>
import DriveInfo from "../components/CloudDrive/DriveInfo.vue";
import DriveFile from "../components/CloudDrive/DriveFile.vue";

import { ElCard } from "element-plus";
</script>

<template>
  <ElRow>
    <ElCol :span="24">
      <ElCard>
        <DriveInfo />
        <DriveFile />
      </ElCard>
    </ElCol>
  </ElRow>
</template>
