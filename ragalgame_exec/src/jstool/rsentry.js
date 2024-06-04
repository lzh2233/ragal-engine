class RalBackground{
    constructor(that){
        this.url = that.link.inner
        this.title = that.link.txt
        this.style = that.style
    }
}

class RalLink{
    constructor(that){
        this.url = that.Link.inner
        /**
         * @type {string}
         */
        let txt = that.Link.txt
        let temp = txt.split('#', 2)
        this.txt = temp[0] ?? ''
        if (temp.length == 2){
            this.opt = temp[1].trim()
        }else{
            this.opt = 'switch'
        }
    }
}

export class Role{
    constructor(that){
        /**
         * @type {string}
         */
        this.name = that.name
        /**
         * @type {string | undefined}
         */
        this.state = that.state ?? "default"
    }

    /**
     * 
     * @param {Role} that 
     * @returns {boolean}
     */
    eq(that){
        if(this.name === that.name){
            return this.state === that.state
        }
        return false
    }
}

class RalDialog{
    constructor(that){
        /**
         * @type {Role|null}
         */
        this.speaker = that.speaker? new Role(that.speaker) : null
        /**
         * @type {string}
         */
        this.speech = that.speech
    }
}

class RalCommand{
    constructor(that){
        this.cmd = that.cmd
    }
}

class OptionCondition{
    constructor(that){
        this.ret_val = that.ret_val
        /**
         * @type {string}
         */
        this.inner = that.inner
    }
}

class OptionItem{
    /**
     * 
     * @param {object} that 
     * @param {Array} that.conditions
     */
    constructor(that){
        this.link = new RalLink(that.val)
        this.conditions = that.conditions.map((val)=>{
            return new OptionCondition(val)
        })
    }
}



class RalOptionList{
    constructor(that){
        /**
         * @type {Array<OptionItem>}
         */
        this.inner = []
        for(let option of that){
            this.inner.push(new OptionItem(option))
        }
    }
}

class RalCustomScript{
    /**
     * 
     * @param {string} script 
     */
    constructor(that){
        if(Reflect.has(that, 'Js')){
            this.type = 'js'
            this.script = "var global = window.__RAGAL__.g;\n" + that.Js
        }else if(Reflect.has(that, 'MarkDown')){
            this.type = 'markdown'
            this.script = that.MarkDown
        }
    }
}

export{RalBackground, RalCommand, RalDialog, RalLink, RalOptionList, OptionItem, RalCustomScript}