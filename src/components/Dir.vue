<script setup lang="ts">
import { reactive, onMounted } from "vue";
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
import { invoke } from "@tauri-apps/api/tauri";

const dirs = reactive<String[]>([]);

function getDirs(path: string): Promise<string[]> {
  return invoke<string[]>('run', { path })
  .catch(err => {
    console.log(err)
    return []
  })
}

onMounted(() => {
  getDirs('/').then(res => {
    dirs.push(...res.map(item => item.slice(1)))
  })
})

</script>

<template>
  <div class="flex-1 m-103">
    <div v-for="item in dirs">
      {{ item }}
    </div>
  </div>
</template>

