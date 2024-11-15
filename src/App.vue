<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
//
// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }
interface todoItem {
  text: string;
  done: boolean;
}

type todoList = todoItem[]

// @ts-ignore unused
const todoList = ref<todoList>([
  {
    text: "Learn Vue 3",
    done: false
  },
  {
    text: "Build a Tauri app",
    done: false
  },
  {
    text: "Write documentation",
    done: false
  },
  {
    text: "Test the application",
    done: true
  }
])
const todoContentInput = ref("")

async function removeTodo(index: number) {
  todoList.value = await invoke("remove_todo", {index: index})
}

// @ts-ignore
async function addTodo() {
  todoContentInput.value = await invoke("add_todo", {content: todoContentInput.value})
}
</script>

<template>
  <div class="flex-col max-w-[300px] mx-auto mt-16">
    <h1 class="text-3xl">Just Tauri Todo</h1>
    <span>a simple todo app</span>
    <div class="mt-3 flex-row">
      <input type="text" class="w-[80vw] max-w-[280px] rounded pl-1 pr-1" placeholder="Press Enter to add a new todo"
             v-model="todoContentInput"
             @keydown.enter="addTodo()">
    </div>
    <div class="mt-2">
      <div v-for="(item, index) in todoList" :key="index" class="w-[80vw] max-w-[280px] relative">
        <input type="checkbox" v-model="item.done" class="ml-2 mr-2">
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