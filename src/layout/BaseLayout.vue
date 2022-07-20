<script setup>
import { RouterView } from "vue-router";
import { ElContainer, ElAside, ElMain, ElFooter } from "element-plus";
import MusicPlayer from "../components/MusicPlayer.vue";
import { Search, Cloudy } from "@element-plus/icons-vue";
</script>

<template>
  <div>
    <ElContainer>
      <ElContainer>
        <ElAside width="200px" style="height: 100vh">
          <el-menu style="height: 80vh" router :default-active="$route.path">
            <el-menu-item index="/search">
              <el-icon>
                <Search />
              </el-icon>
              <span>音乐搜索</span>
            </el-menu-item>
            <el-menu-item index="/drive">
              <el-icon>
                <Cloudy />
              </el-icon>
              <span>音乐云盘</span>
            </el-menu-item>
          </el-menu>
        </ElAside>
        <ElMain>
          <RouterView />
        </ElMain>
      </ElContainer>

      <ElFooter style="padding-left: 0">
        <MusicPlayer />
      </ElFooter>
    </ElContainer>
  </div>
</template>
