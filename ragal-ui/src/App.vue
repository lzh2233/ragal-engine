<template>
    <div class="outter-box">
        <h1>Ragal</h1>
        
        <div @click="open_project" class="button">open project</div>
        <div @click="new_project" class="button">create new project</div>
        <div @click="bundle_project" class="button">bundle project</div>
        <div @click="exit" class="button">exit</div>

    </div>
</template>

<script setup>
import {invoke, process} from "@tauri-apps/api";
import { open, confirm } from "@tauri-apps/api/dialog";

async function open_project() {
    let path = await open({
        title: "选择要打开的工程文件(夹)",
        filters: [{
            name: "YAML",
            extensions: ["yml", "yaml", "conf"]
        }]
    })
    if (path == null) return
    await invoke("open_project", {path})
        .catch(e => confirm(e, {type: "error", title: "打开错误💦"}).then(ok => ok ? open_project(): null))
}

async function new_project() {
    let path = await open({
        title: "选择创建工程所在的文件夹",
        directory: true
    })
    if (path == null) return
    await invoke("new_project", {path})
        .catch(e => confirm(e, {type: "error", title: "创建错误💦"}).then(ok => ok ? new_project(): null))
}

async function bundle_project() {
    let path = await open({
        title: "选择要打包的工程文件(夹)",
        directory: false,
        filters: [{
            name: "YAML",
            extensions: ["yml", "yaml", "conf", "cfg"]
        }]
    })
    if (path == null) return
    invoke("bundle_project", {path})
        .catch(e => confirm(e, {type: "error", title: "打包错误💦"}).then(ok => ok ? bundle_project(): null))
}

async function exit(){
    await process.exit(0)
}
</script>

<style scoped>
.outter-box{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    height: 60%;
    width: 40%;
}
.outter-box h1 {
    font-size: 20vh;
    font-weight: bolder;
    margin-top: 0;
    margin-bottom: 12%;
}

.outter-box div {
    line-height: 7vh;

    font-weight:bold;
    font-size: 3vh;

    height: 7vh;
    width: 60%;

    text-align: center;

    border: 4px solid rgba(84, 84, 84, 0.7);
    border-radius: 3px;
    margin-bottom: 5%;
    
}
.button{
    cursor: pointer;
}

.button:active{
    border-color: black;
}

</style>