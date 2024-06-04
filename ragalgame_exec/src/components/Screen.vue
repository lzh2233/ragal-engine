<script setup>
import Options from "./Options.vue"
import Dialogue from "./Dialogue.vue";
import Character from "./Character.vue";
import Menu from "./Menu.vue";
import Background from "./Background.vue"

import {ralExec} from "../jstool/screen"
import {GalBook, StandingManager, toLegalPath, toAbsolutePath} from "../jstool/rswrap"
import {RalCommand,RalCustomScript, RalDialog, RalLink, RalOptionList, Role, RalBackground} from "../jstool/rsentry"

import { invoke} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri"
import { onMounted, ref, onUnmounted, computed} from "vue";

import pathBrowserify from "path-browserify";


// import {Ref} from "vue"

const props = defineProps(['src_config'])
const emits = defineEmits(['backtitle'])


window.src_config = null
window.game_config = null
window.cfg_dir = ""


let g = {}
window.__RAGAL__.g = {}
/**
 * @type {Ref<Array<GalBook>>}
 */
const bookShelf = ref([])

const characterShot = ref({})
let ref_shot = null

const is_option_show = ref(false)

const curSpeaker = ref({name: '', speech: ''})
/**
 * @type {Ref<GalBook | null>}
 */
const topBook = ref(null)

const currentDirname = computed(()=>{
  if(topBook.value == void(0)) return ''
  return topBook.value.dir
})

bookShelf.value.popWhenEnd = function(){
  if (this.at(-1)&&this.at(-1).isEnd()){
    this.pop()
    topBook.value = this.at(-1)
    if(topBook.value) background.value.loadImg(topBook.value.curBg())
  }
}

// let exe_pth = null

const menu = ref()
const background = ref()

const cur_bg_style = ref({ opacity: '1' })

/**
 * @type {Ref<Array<RalLink>>}
 */
const option_list = ref([])

/* 
key-value结构：
  name : { shot-name1 : style, shot-name2 : style,... }
*/
let standingManager = new StandingManager()


let stand_cfg = null  // 角色与资源路径映射的配置文件 TODO

// 用来阻塞poll事件
let can_poll = true  


let chipBuffer = {
  buffer: [],
  chipIn,
  finish(){
    if(topBook.value == null) return
    while(this.buffer.length != 0 ){
      let value = this.buffer.pop()
      topBook.value.buffer().unshift({type_name: 'Dialog', value})
    }    
  }
}

onMounted(async () => {
  window.src_config = await getSrcConifg()
  window.game_config = await getGameConfig()
  window.cfg_dir = await currentCfgDir()

  document.onkeyup = e => {
    if (e.code == "Escape" && menu.value != void(0) && !is_option_show.value){
      let {is_show, onswitch} = menu.value
      onswitch(!is_show)
    }
  }

  let entryBook = new GalBook(src_config.entry)
  bookShelf.value.push(entryBook)
  await entryBook.open()
    .then(() =>  entryBook.readExp()) //加载入口md文件
    .catch(err => console.error(err))
  stand_cfg = pathBrowserify.join(src_config.standing, "alias.json")
  let stand_json = await readJson(stand_cfg).catch(() => null) ?? {}  


  await import(toLegalPath(cfg_dir, "init.js"))
    .then(async mod=> {
      if(typeof mod.default == 'function')
        g = await mod.default()
      window.__RAGAL__.g = g

    })
    .catch(err=> console.error(err))
    
  // 立绘资源映射建立
  for (let name in stand_json) {
    let state_src = stand_json[name]
    if (typeof(state_src) == "string"){
      standingManager.set({name}, toLegalPath(src_config.standing, state_src))
    }else if (typeof(state_src) == "object"){
      for(let state in state_src){
        standingManager.set({name, state}, toLegalPath(src_config.standing, state_src[state]))
      }
    }
  }
  topBook.value = entryBook
  await poll()
})

onUnmounted(()=>{
  // global = void(0)
  g = {}
})

async function onback(){
  emits('backtitle')
  window.__RAGAL__.g = null
}


async function readJson(pth) {
  let json = await invoke('open_file', {pth: pth})
  return JSON.parse(json)
}

// async function readFile(pth) {
//   let res = await invoke('open_file', {pth})
//   return res
// }

async function getSrcConifg(){
  let res = await invoke('src_config')
  return res
}
async function getGameConfig(){
  return await invoke('game_config')
} 

/**
 * @returns {Promise<string>}
 */
async function currentCfgDir() {
  return await invoke("current_cfg_dir")
}

//poll事件会被根div的click事件触发时回调
//推动解析md得到的命令流
async function poll(){
  if (!can_poll){
    return
  }
  if (topBook.value == null){
    return
  }
  bookShelf.value.popWhenEnd()
  if (bookShelf.value.length == 0){
    curSpeaker.value = {speech: "/** 已经到底了 **/"}
    return
  }
  const event = topBook.value.next()
  if(event == null) return
  switch(event.type_name){
    case 'OptionList':
      option_list.value = []
      const list = new RalOptionList(event.value)
      for(let item of list.inner){
        if(item.conditions.length == 0){
          option_list.value.push(item.link)
          continue
        }
        let link = item.link
        for(let cond of item.conditions ){
          if(topBook.value.stateCustom().retExec(cond.inner)){
            let t = window.global
            window.global = window.__RAGAL__.g
            let link_t = await import(toLegalPath(topBook.value.dir,cond.ret_val)).then(mod => {
              if(typeof mod.default != "function") return Promise.reject("export default type isn't function")
              return mod.default.call(topBook.value.state.custom.state)
            })
            .then(val => {
              window.global = t
              return val
            })
            .catch(err => {
              console.error(err)
            })
            if (link_t != void(0)){
              let url = link_t.url 
              if(!pathBrowserify.isAbsolute(link_t.url))
                url = pathBrowserify.join(pathBrowserify.dirname(cond.ret_val), url)
              link = {
                url: link_t.url ? url : link.url,
                txt: link_t.txt ?? link.txt,
                opt: link_t.opt ?? link.opt
              }
            }
            option_list.value.push(link)
            break
          }
        }
      }
      is_option_show.value = true
      break
    case 'Dialog':
      const dialog = new RalDialog(event.value)
      curSpeaker.value.name = dialog.speaker ? dialog.speaker.name : ''
      curSpeaker.value.speech = dialog.speech
      if(curSpeaker.value.name.toLowerCase() == "os"){
        curSpeaker.value.name = ''
        
        break
      }
      let shot = topBook.value.state.shotsManager.get_shot(curSpeaker.value.name)
      if(ref_shot == shot){
        characterShot.value[curSpeaker.value.name].src = standingManager.get(dialog.speaker)
        break
      }
      let tmp_shot = {}
      for(let name in shot){
        if(dialog.speaker.name != name)
          tmp_shot[name] = {style : shot.get_style(name), src: standingManager.get(new Role({name}))}
        else
          tmp_shot[name] = {style : shot.get_style(name), src: standingManager.get(dialog.speaker)}
      }
      ref_shot = shot
      characterShot.value = tmp_shot
      break
    case 'Linker':
      if(Reflect.has(event.value, 'Image')){
        const img = new RalBackground(event.value.Image)
        topBook.value.setCurBg(img.url)
        if(background.value.loadImg(topBook.value.curBg()))await poll()
      }else{
        const link = new RalLink(event.value)
        await changebranch(link)
      }
      break
    case 'CustomScript':
      const custom_script = new RalCustomScript(event.value)
      if(custom_script.type == 'js')
        await topBook.value.state.custom.exec(custom_script.script ?? '', topBook.value.dir, {switchMD, callMD, chipBuffer})
      await poll()
      break
    case 'RalCommand':
      const ralcmd = new RalCommand(event.value)
      ralExec.call({
        state: topBook.value.state,
        gameconfig: game_config
      }, ralcmd)
      await poll()
      break
  }
}


/**
 * @param {RalLink} link
 */
async function changebranch(link){
  if(link.url == null){
    await poll()
  }else{
    let path = toAbsolutePath(topBook.value.dir, link.url)
    if (link.opt == 'call'){
      await callMD(path)
    }else if(link.opt == 'switch'){
      await switchMD(path)
    }else if(link.opt == 'custom'){
      await customChoose(path)
    }
  }
  await poll()
}

/**
 * @param {Role} speaker
 * @param {string} speech
 */
function chipIn(speaker, speech){
  if(typeof speaker  == 'string') speaker = {name: speaker, state: null} 
  this.buffer.push({speaker, speech})
  return this
}

/**
 * 选择要切换的文本，返回解析出来的命令流   
 * @param {string} path
 */ 
async function switchMD(path) {
  if(path != null && path != ''){
    let old_book = bookShelf.value.pop()
    if(old_book != null){
      let book = new GalBook(path)
      await book.open()
      await book.readExp(200)
      curSpeaker.value = {name: '', speech: ''}
      characterShot.value ={}
      bookShelf.value.push(book)
      topBook.value = book
    }
  }
  // is_option_show.value = false

}


/**
 * @param {string} path
 */
async function callMD(path) {
  let book = new GalBook(path)
  await book.open()
  await book.readExp(50)
  curSpeaker.value = {name: '', speech: ''}
  characterShot.value = {}
  bookShelf.value.push(book)
  topBook.value = book

}

/**
 * @param {string} pth
 */
async function customChoose(pth){
  if (pathBrowserify.extname(pth) != '.js')return Promise.reject('the file without `js` extname #custom')
  let path = convertFileSrc(pth)
  let t = window.global
  window.global = window.__RAGAL__.g
  await import(path).then( mod => {

    if(typeof mod.default == 'function'){
      return mod.default.call(topBook.value.state.custom.state, pathBrowserify.dirname(pth), {callMD, switchMD, chipBuffer})
    }else{
      return Promise.reject("export default type isn't function")
    }
  })
  .then(() => window.global = t)
  .catch(err => console.error(err))
}

async function load(){
  can_poll = true
  await poll().then(()=>{
    cur_bg_style.value.opacity = '1'
  })
}
function beforeLoad(){
  cur_bg_style.value.opacity = '0'
  can_poll = false
}

</script>

<template>
  <div class="screen" @click="poll">
    <Background  @load="load" @beforeload="beforeLoad" ref="background"/>
    <Character :shots="characterShot" :speaker-name="curSpeaker.name" :style="cur_bg_style"/>
    <Dialogue :speaker="curSpeaker" :dirname="currentDirname"/>
    <Options :is_show="is_option_show" :optionlist="option_list" @choose="async(link) => {is_option_show = false; await changebranch(link)}" />
    <Menu @click-backtitle="onback" :ref = 'el => menu=el '/>
  </div>
</template>

<style scoped>
/* .menu{
  position: absolute;
  top: 0;
  right: 0;
} */

.screen {
  display: flex;
  position: relative;
  overflow: hidden;

  height: 100vh;

  background-color: rgb(0, 0, 0);

  align-items: center;
  justify-content: center;

}
</style>