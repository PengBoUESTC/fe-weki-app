<script setup lang="ts">
import { computed, toRefs } from "vue";
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// import { invoke } from "@tauri-apps/api/tauri";
import { Directory } from '../types/Directory'

interface Props {
  focusDir: string;
  dirList: Directory[];
}

interface Emits {
  (event: 'dirClick', data: Directory, isOpen: boolean): void
}

const props = defineProps<Props>()
const emits = defineEmits<Emits>()


function handleDirClick(dir: Directory) {
  emits('dirClick', dir, isOpen.value)
}

const isOpen = computed(() => {
  const { dirList } = props
  const { parent } = dirList[0]
  if(!parent) return true;
  return props.focusDir.startsWith(parent)
})

</script>

<template>
  <div v-if="!!dirList.length && isOpen" class="dir-list">
    <div
      class="dir-list--item"
      :class="{'dir-list--open': isOpen }"
      v-for="dir in dirList"
      @click.stop="handleDirClick(dir)"
    >
      {{ dir.name }}
      <Dir
        :dirList="dir.children || []"
        :focusDir="focusDir"
        @dirClick="handleDirClick"
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.dir-list {
  &--open {
    color: green;
  }
  &--item {
    padding-left: 0.1rem;
  }
}
</style>
