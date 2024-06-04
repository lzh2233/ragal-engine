<script setup>
import {ref} from "vue"
  const emits = defineEmits(['click-setting', 'click-backtitle'])

  const is_show = ref(false)

  function onswitch(flag){
    is_show.value = flag
  }

  function oncontinue(){
    is_show.value = false
  }
  function onsetting(){
    emits('click-setting')
  }
  function onbacktitle(){
    emits('click-backtitle')
  }


  defineExpose({
    is_show,
    onswitch
  })
</script>


<template>
  <div class="menu" v-show="is_show" @click.stop="">
    <h1>Menu</h1>
    <div class="menu-button"  @click="oncontinue">Resume</div>
    <div class="menu-button"  @click="onsetting">Setting</div>
    <div class="menu-button"  @click="onbacktitle">Return to Title</div>
  </div>
</template>


<style scoped>
  .menu{
    display: flex;
    

    width: 100%;
    height: 100%;
    flex-direction: column;

    justify-content: center;
    /* align-items: center; */

    background-color: rgba(23, 23, 23, 0.95);
    backdrop-filter: blur(3px);
  }

  .menu h1 {
    display: flex;
    position: relative;
    justify-content: center;
    color: rgb(130, 242, 252);
    font-size: 7vh;

    /* top: -10%; */
  }

  .menu-button{
    display: flex;
    font-size: 2.4vh;
    font-weight: bold;
    justify-content: center;
    color: white;
    padding: 5px;
  }

  :hover.menu-button {
    color: black;
    background-color: white;
  }
</style>