<script setup>

const props = defineProps([
  "optionlist", "is_show"
])

defineEmits(['choose'])

</script>

<template>
  <div class="options_face" v-show="is_show" @click.stop="">
    <div v-for="(link,idx) in optionlist" class="option" :key="idx" @click="$emit('choose', link)">{{ link.txt }}</div>
  </div>
</template>

<style scoped>
  .options_face{
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;

    position: relative;

    background-color: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(3px);
    align-items: center;
    justify-content: center;
  }

  .option{
    display: flex;

    margin-top: 4%;
    padding: 2%;
    border: 2px solid rgba(255, 255, 255, 0.8);
    font-size: 1.7vw;
    width: 50%;

    color: white;
    font-weight: bold;
    background-color: rgba(41, 41, 41, 0.8);
  

    justify-content: center;
    align-items: center;
  }

  .option:hover{
    background-color: white;
    color: black;
    cursor: pointer;
  }
</style>