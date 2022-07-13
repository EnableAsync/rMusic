<script setup>
import { ref } from "vue";
const username = ref("EnableAsyncc");
const level = ref("Lv9");
const checked = ref(false);
</script>

<template>
  <el-card style="width: 100%" :body-style="{ width: '350px' }">
    <el-row justify="center" align="middle">
      <el-col :span="8" style="text-align: center">
        <el-avatar
          :size="60"
          src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"
        />
      </el-col>
      <el-col :span="8" style="text-align: center; width: 100%">
        <el-row>
          <div style="margin: auto">
            {{ username }}
          </div>
        </el-row>
        <el-row>
          <div style="margin: auto">
            {{ level }}
          </div>
        </el-row>
      </el-col>
      <el-col :span="8" style="text-align: center">
        <el-button :disabled="checked" @click="checked = !checked">
          {{ checked ? "已签到" : "签到" }}
        </el-button>
      </el-col>
    </el-row>
  </el-card>
</template>
