<script setup>
// import Menu from './components/Menu.vue';
import { onMounted, ref } from 'vue';
import Screen from './components/Screen.vue';
import TitlePage from './components/TitlePage.vue';
import { emit as tauri_emit} from "@tauri-apps/api/event";
const TitleOrPlayer = ref(true)

onMounted(()=> {
  document.onkeydown = async e => {
    if (e.ctrlKey && e.shiftKey && e.code == "i"){
      await tauri_emit("open-devtool")
      e.preventDefault()
    }
  }
})
async function back(){
  TitleOrPlayer.value = true
}

</script>

<template>
  <TitlePage v-if="TitleOrPlayer" @switch="()=>TitleOrPlayer=false"/>
  <Screen @backtitle=back v-else/>
</template>

