<script setup>
import { reactive, watch } from 'vue'

const activeSpeaker = reactive({})


const props = defineProps(['shots', 'speakerName'])

// 提高正在对话者的立绘亮度
function speaking(actor){
  if (actor == null) return
  activeSpeaker[actor] = {filter : `brightness(100%)`}
}

// 降低已讲完者的立绘亮度
function spoken(actor){
  if (actor == null) return
  activeSpeaker[actor] = {}
}

// 删除此文本不涉及的立绘
// watch(() => props.shots, (now) => {
//   for (let i in activeSpeaker){
//     if(now[i] === undefined){
//       delete activeSpeaker[i]
//     }
//   }
// })

watch(()=> props.speakerName, (now, old) => {
  speaking(now)
  spoken(old)
})

</script>

<template>
  <div class="characters" >
    <div class="standing-img" v-for="(value, name) in shots" :key="name" :style="value.style">
      <img :src="value.src" :style="activeSpeaker[name]" >
    </div>
  </div>
</template>

<style scoped>
  .characters{
    display: flex;
    /* position: absolute; */
  }
  .standing-img img{
    filter: brightness(40%);
    height: 100%;

    object-fit: contain;
    transition: filter 0.2s linear;
  }

  .characters .standing-img{
    position: absolute;

    /* width: 30%; */
    height: 100%;  
    padding: 8%;
    padding-bottom: 0%;
    margin: 0%;
  }
</style>