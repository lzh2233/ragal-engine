import {invoke} from "@tauri-apps/api"
import {convertFileSrc} from "@tauri-apps/api/tauri"
import { Role } from "./rsentry"
import {RalCustom} from "./cusrun"
import {resolveAbsolutePath} from "./screen"
import  {isAbsolute} from "path-browserify"

// import {computed} from "vue"

class GalMdReader{

    constructor(){
        this.offset = 0
        this.isLegal = false
        this.content = []
    }
    
    async open(path, offset=0){
        return await readerBuild(path)
            .then(async()=>{
                this.isLegal = true
                this.offset=offset
                await seekFromStart(this.offset)
                return await get_info().then(val=>{
                    this.content = val.content
                    return {
                        dir: val.dir, 
                        fileName: val.file_name, 
                        customState: val.custom_state,
                        len: val.len
                    }
                })
            })
            .catch(err=>{
                this.isLegal = false
                return err
            })
    }
    
    /**
     * 
     * @param {number} expLine 
     * @returns {Promise<Array|null>}
     */
    async readExp(expLine = 200){
        if (this.isLegal == false){
            return null
        }

        let {offset, vector} = await readExp(expLine).catch(err=>{
            alert(err)
        })
        if(offset && vector){
            this.offset = offset
            for(let e of vector){
                this.content.push(e)
            }
            return vector
        }
    }

    position(){
        if (this.isLegal == false){
            return null
        }
        return this.offset
    }
}

class GalBook{
    constructor(path){
        this.reader = new GalMdReader()
        this.path = path
        this.dir = ''
        /**
         * @typedef {Object} RalState
         * @property {ShotManager} shotsManager
         * @property {RalCustom?} custom
         * @property {string} curBg
         */
        /** 
         * @type {RalState}
         */
        this.state = {
            shotsManager: new ShotManager(),
            curBg: '',
        }
    }


    curBg(){
        return this.state.curBg
    }

    setCurBg(val){
        return this.state.curBg = toAbsolutePath(this.dir, val)
    }

    async open(offset=0){
        return this.reader.open(this.path, offset).then((val)=>{
            this.dir = val.dir
            this.fileName = val.fileName
            this.state.custom = new RalCustom(val.customState)
            this.size = val.len
            delete this.path
            this.open = void(0)
        })
    }

    async readExp(expLine=200){
        let entries =  await this.reader.readExp(expLine)
        if (entries == null){
            return
        }

        // for(let entry of entries){
        //     if (entry.type_name == 'Linker' && Reflect.has(entry.value, "Image")){
        //         let e = new RalBackground(entry.value.Image)
        //         this.state.bgQue.push(e.url)
        //     }
        // }
    }

    length(){
        return this.reader.content.length
    }

    next(){
        if (this.isEnd()){
            this.reader.isLegal = false
            return null
        }
        let val = this.buffer().shift()
        return val
    }

    // shift_align(){
    //     for(let i in this.reader.content){
    //         if (i >= this.index){
    //             break
    //         }
    //         this.reader.content.shift()
    //     }
    //     this.index = 0
    // }
    
    buffer(){
        return this.reader.content
    }

    stateCustom(){
        return this.state.custom
    }

    isEof(){
        return this.size == this.reader.offset 
    }

    isEnd(){
        return this.isEof() && this.length() == 0
    }

    // async preLoadBg(exp = 1){
    //     let bg_que = this.state.bgQue
    //     while(exp > 0 ){
    //         let bg = new Image()
    //         let path = bg_que.shift();
    //         if(path != void(0))
    //             bg.src = toLegalPath(this.dir, path)
    //         this.state.next_bg.push(bg)
    //         exp -- 
    //     }
    // }


    // async nextBg(){
    //     this.state.next_bg.shift()
    //     await this.preLoadBg()
    // }
    
}

class Shot{
    
    /**
     * 
     * @param {string} name 
     */
    get_style(name){

        return this[name]
    }

    /**
     * 
     * @param {string} name 
     * @param {object} style 
     */
    set(name, style){
        this[name] = style
    }

}

export class ShotManager{

    constructor(){
        /**
         * @type {Map<string, Shot>}
         */
        this.inner = new Map()
        this.set()
    }

    /**
     * 
     * @param {string} name 
     */
    get_shot(name){
        return this.inner.get(name) ?? new Shot()
    }

    /**
     * 
     * @param {Shot} shot 
     */
    set(name,shot){
        this.inner.set(name, shot)
    }
    delete(name){
        this.inner.delete(name)
    }
    clear(){
        this.inner.clear()
    }
}


class StandingManager{
    /**
     * StandingManager{
     *  "name1" : {
     *      "default" : "path/src/.."     
     *      "state1" : "..."
     *  },
     * ...
     * }
     */
    constructor(){
        /**
         * @type {Map<string, Map<string, string>>}
         */
        this.inner = new Map()
    }

    /**
     * @param {Role} speaker
     */
    get(speaker){
        const role = this.inner.get(speaker.name)
        if (role != void(0)){
            return role.get(speaker.state ?? "default") ?? role.get("default")
        }
    }
    /**
     * 
     * @param {Role} speaker 
     * @param {string} src 
     */
    set(speaker, src){
        if (!this.inner.has(speaker.name)){
           this.inner.set(speaker.name, new Map())
        }
        let standing = this.inner.get(speaker.name)
        standing.set(speaker.state ?? "default", src)
    }

    delete(name){
        this.inner.delete(name)
    }
}

/**
 * 
 * @param {number} expLine 
 * @returns {Promise<Array>}
 */
async function readExp(expLines){
    return await invoke("read_exp", {expLines: expLines})
}

/**
 * 
 * @param {string} path 
 */
async function readerBuild(path){
    await invoke("reader_new", {path: path})
}

/**
 * 
 * @returns {Promise<object>}
 */
async function get_info(){
    return await invoke("get_info")
}

/**
 * 
 * @param {number} start 
 */
async function seekFromStart(start){
    await invoke("seek_from_start", {start: start})
}



function toLegalPath(dir,p){
    return convertFileSrc(toAbsolutePath(dir, p))
}

/**
 * 
 * @param {string} dir 
 * @param {string} p  
 */
function toAbsolutePath(dir, p){
    let pth = decodeURI(p)
    if(pth.startsWith("$PROJECTROOT")) {

        return cfg_dir + p.trimStart("$PROJECTROOT")
    }
    if(isAbsolute(pth)) return pth
    return resolveAbsolutePath(`${dir}/${pth}`)
}

export {GalBook, StandingManager, Shot, toLegalPath, toAbsolutePath}