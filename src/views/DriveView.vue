<script setup>
import DriveInfo from "../components/CloudDrive/DriveInfo.vue";
import DriveFile from "../components/CloudDrive/DriveFile.vue";

import { ElCard } from "element-plus";
</script>

<template>
  <main>
    <ElCard style="width: 100%">
      <DriveInfo />
      <DriveFile />
    </ElCard>
  </main>
</template>
