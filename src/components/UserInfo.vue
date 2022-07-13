<script setup>
import { ref } from "vue";
import { ElCard, ElRow, ElCol, ElAvatar, ElButton } from "element-plus";

const username = ref("EnableAsyncc");
const level = ref("Lv9");
const checked = ref(false);
</script>

<template>
  <ElCard style="width: 350px">
    <ElRow :gutter="20" justify="center" align="middle">
      <ElCol :span="8" style="text-align: center">
        <ElAvatar
          :size="60"
          src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"
        />
      </ElCol>
      <ElCol :span="8" style="text-align: center; width: 100%">
        <ElRow>
          <div style="margin: auto">
            {{ username }}
          </div>
        </ElRow>
        <ElRow>
          <div style="margin: auto">
            {{ level }}
          </div>
        </ElRow>
      </ElCol>
      <ElCol :span="8" style="text-align: center">
        <ElButton :disabled="checked" @click="checked = !checked">
          {{ checked ? "已签到" : "签到" }}
        </ElButton>
      </ElCol>
    </ElRow>
  </ElCard>
</template>
