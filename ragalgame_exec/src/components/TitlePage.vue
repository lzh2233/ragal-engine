<script setup>
import { onMounted, onUnmounted} from 'vue'
import {invoke} from "@tauri-apps/api"

const emit = defineEmits(['switch'])

window.__RAGAL__.onStart = () => {
	emit("switch")
}

window.__RAGAL__.onExit = async () => {
	await(await window.__TAURI__.window.getCurrent()).close()
}


let script = document.createElement("script")
onMounted(async() => {

	script.innerHTML = `\
	function onStart() {window.__RAGAL__.onStart()}
	async function onExit() {await window.__RAGAL__.onExit()}
	`
	document.head.appendChild(script)
	let el = document.querySelector("#page_postion")
	let page_html = await invoke("custom_html", {kind : "titlePage"}).catch(err=> alert(err))
	el.innerHTML = page_html
})
onUnmounted(()=>{
	document.head.removeChild(script)
})
</script>

<template>
	<div id="page_postion"></div>
</template>