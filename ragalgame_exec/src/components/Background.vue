<template>
    <img :src="src" :style="style" @load="() => {$emit('load');style.opacity='1'}" class="background">
  </template>
  
  <style scoped>
  .background {
    position: absolute;
  
    height: 100%;
    width: auto;
    object-fit: cover;
  
    background-repeat: no-repeat;
    background-position: center center;
    background-size: cover;
  
    transition: opacity 0.4s ease;
  
  }
  </style>
  
  <script setup>
  import {ref} from "vue"
  import {convertFileSrc} from "@tauri-apps/api/tauri"
  
  const emits = defineEmits(['beforeload', 'load'])
  // const props = defineProps(['src'])
  
  
  
  
  const src = ref('')
  const speed = 400
  const style = ref({
    opacity: '1'
  })
  
  function loadImg(img_src){
    let img = convertFileSrc(img_src)
    if (img == src.value){
      return true
    }else{
      style.value.opacity = '0'
      emits('beforeload')
      setTimeout(()=>{
        src.value = img
      }, speed)
      return false 
    } 
  }
  
  defineExpose({
    loadImg
  })
  
  </script>