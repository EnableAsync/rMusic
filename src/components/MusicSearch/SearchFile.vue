<script setup>
import { ElTable, ElTableColumn, ElInput, ElButton } from "element-plus";
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api";

const search = ref("不为谁而作的歌");
let tableData = reactive([]);

const handleSearch = (name) => {
  invoke("search_music", { name: name }).then((response) => {
    console.log(response);
    tableData.push(response);
  });
};

const handleEdit = (index, row) => {
  console.log(index, row);
};
const handleDelete = (index, row) => {
  console.log(index, row);
};

const formatter = (row) => {
  console.log(row);
};
</script>

<template>
  <ElTable :data="tableData" style="width: 100%">
    <ElTableColumn label="音乐标题" prop="song.name" />
    <ElTableColumn label="歌手" prop="song.artists[0].name" />
    <ElTableColumn label="URL" prop="song.context.HQ" />
    <ElTableColumn align="right">
      <template #header>
        <ElInput
          v-model="search"
          size="small"
          @keyup.enter="handleSearch(search)"
          placeholder="搜索音乐"
        />
      </template>
      <template #default="scope">
        <ElButton size="small" @click="handleEdit(scope.$index, scope.row)"
          >播放</ElButton
        >
        <ElButton size="small" @click="handleDelete(scope.$index, scope.row)"
          >上传到云盘</ElButton
        >
      </template>
    </ElTableColumn>
  </ElTable>
</template>
