<script setup>
import {
  ElTable,
  ElTableColumn,
  ElInput,
  ElButton,
  ElRow,
  ElCol,
  ElMessage,
} from "element-plus";
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api";
import "vue3-audio-player/dist/style.css";
import { Search } from "@element-plus/icons-vue";
import { usePlayerStore } from "../../stores/player";

const search = ref("不为谁而作的歌");
let tableData = reactive([]);
const player = usePlayerStore();
const searching = ref(false);

const handleSearch = (name) => {
  searching.value = true;
  invoke("search_music", { name: name }).then(
    (response) => {
      tableData.splice(0, tableData.length);
      tableData.push(...response);
      console.log(tableData);
    },
    (err) => {
      console.log(err);
      ElMessage(err);
    }
  );
  searching.value = false;
};

const handlePlay = (index, row) => {
  console.log(row);
  player.play({
    src: row.retrieved.url,
    title: row.song.song.name,
    coverImage: "",
  });
};
const handleDelete = (index, row) => {
  console.log(index, row);
};
</script>

<template>
  <ElRow style="align-items: center" justify="space-evenly">
    <ElCol :span="4">
      <span style="font-size: 1.5em; font-weight: bold">音乐搜索</span>
    </ElCol>
    <ElCol :span="16">
      <ElInput
        v-model="search"
        @keyup.enter="handleSearch(search)"
        placeholder="搜索音乐"
        style="margin: auto"
        :prefix-icon="Search"
      />
    </ElCol>
    <ElCol :span="2">
      <ElButton :loading="searching" @click="handleSearch(search)">
        搜索
      </ElButton>
    </ElCol>
  </ElRow>
  <ElTable :data="tableData" style="width: 100%">
    <ElTableColumn label="音乐标题" prop="song.song.name" />
    <ElTableColumn label="歌手" prop="song.song.artists[0].name" />
    <ElTableColumn label="URL" prop="retrieved.url" />
    <ElTableColumn label="来源" prop="retrieved.source" />
    <ElTableColumn align="right">
      <template #header>
        <ElInput
          v-model="search"
          @keyup.enter="handleSearch(search)"
          placeholder="搜索音乐"
        />
      </template>
      <template #default="scope">
        <ElButton @click="handlePlay(scope.$index, scope.row)">播放</ElButton>
        <ElButton @click="handleDelete(scope.$index, scope.row)"
          >上传到云盘</ElButton
        >
      </template>
    </ElTableColumn>
  </ElTable>
</template>
