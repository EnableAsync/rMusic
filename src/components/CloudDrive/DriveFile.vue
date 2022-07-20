<script setup>
import { ElTable, ElTableColumn } from "element-plus";
import { computed, ref } from "vue";

const search = ref("");
const filterTableData = computed(() =>
  tableData.filter(
    (data) =>
      !search.value ||
      data.name.toLowerCase().includes(search.value.toLowerCase())
  )
);
const handleEdit = (index, row) => {
  console.log(index, row);
};
const handleDelete = (index, row) => {
  console.log(index, row);
};

const tableData = [
  {
    name: "烟雨行舟",
    author: "司南",
    type: "mp3",
    date: "2022-06-06",
  },
];
</script>

<template>
  <ElTable :data="filterTableData" style="width: 100%">
    <ElTableColumn label="音乐标题" prop="name" />
    <ElTableColumn label="歌手" prop="author" />
    <ElTableColumn label="格式" prop="type" />
    <ElTableColumn label="上传时间" prop="date" />
    <ElTableColumn align="right">
      <template #header>
        <el-input v-model="search" placeholder="搜索云盘" />
      </template>
      <template #default="scope">
        <el-button @click="handleEdit(scope.$index, scope.row)">下载</el-button>
        <el-button type="danger" @click="handleDelete(scope.$index, scope.row)"
          >删除</el-button
        >
      </template>
    </ElTableColumn>
  </ElTable>
</template>
