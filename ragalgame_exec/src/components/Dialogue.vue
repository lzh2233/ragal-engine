<script setup>
import { ref, watch, onUnmounted } from 'vue';
import { toLegalPath } from '../jstool/rswrap';

const props = defineProps(['speaker', 'dirname'])
           
const speechSpan = ref()           
let step = 5;        
let animateId = null; 
let spanText = document.createElement('div')
let iter = {
  idx : 0,
  ref_array: [],
  next(){
    if(iter.ref_array.length <= this.idx)return null
    let e = this.ref_array[this.idx]
    if(typeof e.data == 'string'){
      this.ref_array[this.idx].data = e._data.slice(0, e.cusor)
      if(e.cusor > e._data.length) this.idx ++ ;
      e.cusor += 1;
      return
    }
    e.style.display = 'inline'
    this.idx ++ ;
  }
}

function init(node){
  let res = []
  for(let e = node; e != null; e = e.nextSibling){
    if(e.data != void(0) && typeof e.data == 'string'){
      e._data = e.data
      e.data = ''
      e.cusor = 0
      res.push(e)
      continue
    }
    if(e.firstChild == void(0)){
      e.style.display = 'none'
      res.push(e)
      continue
    }
    for(let x of init(e.firstChild))res.push(x)
    
  }
  return res
}

// 监视对话内容的变化
watch(() => props.speaker.speech, (newSpeech) => {
  speechSpan.value.innerHTML = ''
  iter.idx = 0
  let rule = /src\s*=\s*"([\s\S]*?)"/gm
  let speech = newSpeech.replace(rule, (_, p1)=>{
    return `src='${toLegalPath(props.dirname, p1.trim())}'`
  })
  startTyping(speech,0)         
});

onUnmounted(() => {
  cancelAnimationFrame(animateId)
});
// 开始逐字显示对话内容

// 逐字动画的核心代码
/**
 * 
 * @param {string} newSpeech 
 * @param {number} cnt 
 */
function startTyping(newSpeech, cnt){
  cancelAnimationFrame(animateId)
  if (step <= 0) {
    step = 1;
  }     
  spanText.innerHTML = newSpeech
  iter.ref_array = init(spanText.firstChild)
  let typing = () => {

    // step为刷新间隔
    if( cnt % step == 0){
      iter.next()
      speechSpan.value.innerHTML = spanText.innerHTML
    }

    if (iter.idx < iter.ref_array.length){
      animateId = requestAnimationFrame(typing)
    }else{
      cancelAnimationFrame(animateId)
    }
    cnt ++ ;
  }
  typing()
}

</script>

<template>
  <div class="dialogue">
    <div class="speaker-name">{{ speaker.name }}</div>
    <div class="speaker-content" ref="speechSpan"></div>
  </div>
</template>

<style scoped>
.dialogue{
  position: relative;

  width: 100%;
  height: 30%;

  display: flex;
  flex-direction: column;
  align-items: center;

  background: linear-gradient(to top, rgba(0,0,0), rgba(0,0,0, 0.7) 80% ,rgba(0, 0, 0, 0)) ;
  position: absolute;
  bottom: 0px;
  box-sizing: border-box;
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
  
  padding: 10px;

}
.speaker-name{
  width: calc(60% + 100px);
  font-weight: bold;
  height: 3.4vh;
  font-size: 3.3vh;
  color: white;
  margin-bottom: 9px;
  border-bottom: 2px solid rgba(151, 151, 151, 0.8);
  padding-bottom: 10px;
  padding-left: 8px;
}

.speaker-content{
  width: calc(60% + 100px);
  height: 75%;
  display: inline-block;
  word-break: break-all;
  color: white;
  font-size: 3.1vh;
  line-height: 1.6;
  resize: both;
}
</style>