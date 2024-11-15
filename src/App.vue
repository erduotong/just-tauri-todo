<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

interface todoItem {
  text: string;
  done: boolean;
}

type todoList = todoItem[]

// @ts-ignore unused
const todoList = ref<todoList>([])
const todoContentInput = ref("")

async function removeTodo(index: number) {
  todoList.value = await invoke("remove_todo", {index: index})
}

// @ts-ignore
async function addTodo() {
  if (todoContentInput.value.trim() === "" ) {
    return
  }
  todoList.value = await invoke("add_todo", {content: todoContentInput.value}) as todoList
}

async function updateTodoDoneStatus(index: number, done: boolean) {
  todoList.value = await invoke("update_done_status", {index: index, done: done}) as todoList
}
onMounted(async () => {
  todoList.value = await invoke("init")
})
</script>

<template>
  <div class="flex-col max-w-[300px] mx-auto mt-16">
    <h1 class="text-3xl underline">Just Tauri Todo</h1>
    <span class="text-gray-500">a simple todo app</span>
    <div class="mt-3 flex-row border-b-gray-200 border-b-2">
      <input type="text" class="w-[80vw] max-w-[280px] rounded pl-1 pr-1" style="outline:medium;"
             placeholder="Press Enter to add a new todo"
             v-model="todoContentInput"
             @keydown.enter="addTodo()">
    </div>

    <div class="mt-2 ">
      <div v-for="(item, index) in todoList" :key="index" class="w-[80vw] max-w-[280px] relative p-1 m-[2px] rounded border-2">
        <input type="checkbox" v-model="item.done" class="ml-2 mr-2" @change="updateTodoDoneStatus(index,item.done)">
        <span :class="item.done ? 'line-through text-gray-500' : ''"
              class="transition-all duration-500 ease-in-out absolute max-w-[200px] break-words">{{ item.text }}</span>
        <span
            class="cursor-pointer transition-all duration-500 ease-in-out absolute right-0 left-auto text-[12px] opacity-50 hover:opacity-100"
            @click="removeTodo(index)"
        >✖</span>
      </div>
    </div>
  </div>
</template>

<style scoped>


</style>
<style>

</style>