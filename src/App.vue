<script setup lang="ts">
import { reactive, onMounted, ref } from "vue";
import { isEmpty } from 'lodash'

// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { getDirs } from './utils/getDirs'

import Dir from "./components/Dir.vue";
import type { Directory } from './types/Directory'


const dirs = reactive<Directory[]>([]);
const focusDir = ref("/")

onMounted(() => {
  getDirs('/').then(res => {
    dirs.push(...res.map(item => {
      return {
        name: item.slice(1),
        path: item,
        parent: null
      }
    }))
  })
})

function handleDirClick(dir:Directory, isOpen?: boolean) {
  const { path, parent } = dir
  focusDir.value = isOpen ? path : parent || '/'
  getDirs(path).then(res => {
    dir.children = res.map(item => {
      return {
        name: item.slice(path.length + 1),
        path: item,
        parent: path,
      }
    })
    if(!res.length) {
      focusDir.value = parent || '/'
    }
  })
}
</script>

<template>
  <div class="container">
    <Dir :dir-list="dirs" :focusDir="focusDir" @dirClick="handleDirClick"/>
  </div>
</template>

<style scoped>
</style>
