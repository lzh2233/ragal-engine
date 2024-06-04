class RalCustom{
    /**
     * 
     * @param {object} state 
     */
    constructor(state){
        this.state = state
        /**
         * @type {Function}
         */
        this.exec = function(script, dirname ,shelf){ 
            window.global = window.__RAGAL__.g
            const fn =  (new Function('dirname','shelf', script)).bind(this.state)
            return fn(dirname, shelf)
        }
        /**
         * @type {Function}
         */
        this.retExec = function(script){return (eval(script))}.bind(this.state)
        
    }
}


export {RalCustom}