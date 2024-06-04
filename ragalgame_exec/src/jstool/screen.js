import {ShotManager, Shot} from "./rswrap"

export function getFileNameFromPath(path) {
  const parts = path.split(/[\\\/]/);
  return parts[parts.length - 1];
}
  
  
// 将非法逻辑的路径转化为合法的路径
export function resolveAbsolutePath(absolutePath) {
  const paths = absolutePath.split((/[\\\/]/));
  const resolvedPaths = [];

  for (const path of paths) {
    if (path === '..') {
      resolvedPaths.pop(); // 移除上一级目录
    } else {
      resolvedPaths.push(path); // 添加当前路径
    }
  }
  return resolvedPaths.join('/')
}

/**
 * @typedef {Object} RalState
 * @property {ShotManager} shotsManager
 * @property {Array} bgQue
 * @property {RalCustom?} custom
 * @property {Array<HTMLImageElement>} next_bg 
 */
/**
 * @typedef {Object} GameConfig
 * @prop {Object} standing
 */
/**
 * @typedef {object} ThisTrait
 * @prop {RalState} state 
 * @prop {GameConfig} gameconfig 
 */

/**
 * 
 * @param {RalCommand} rcmd
 * @this ThisTrait 
 */
export function ralExec(rcmd){
  let m;
  let re;
  let cmd = rcmd.cmd
  if (m = /(?<=\\\+\s*{)[^}]+(?=})/g.exec(cmd)) {
    // 增加镜头命令，example：+{mdl : (x, y, zoom), ...}
    // 注意，本命令的regex识别是支持默认镜头参数添加的，example：+{mdl, ...}，但是目前不支持TODO
    re = /(?<name>[^:\s\,，]+)\s*([:：]\s*\(\s*(?<left>[-+]?\d+[\w%]*)\s*,\s*(?<bottom>[-+]?\d+[\w%]*)\s*,\s*(?<zoom>[-+]?\d+[\w%]+)\s*\))?/g
    let match;
    let shot = new Shot()
    /**@type {Array<RegExpExecArray>} */
    const matchQ = []
    while ((match = re.exec(m[0])) !== null) {
      matchQ.push(match)
    }
    for(let i in matchQ){
      match = matchQ[i]
      let name = match.groups["name"]
      let param;
      if (match[2] != null){
        param = { left: match.groups["left"], bottom: match.groups["bottom"], height: match.groups["zoom"] }
      }else if(matchQ.length < 3){
        let standing = this.gameconfig.standing[matchQ.length][i];
        if(standing != null) param = {left: standing[0], bottom: standing[1], height: standing[2]}
      }
      shot.set(name, param)
      this.state.shotsManager.set(name,shot)
    }
  } else if (m = /(?<=\-\s*{)[^}]+(?=})/g.exec(cmd)) {
    // 去除镜头命令
    re = /[^,，\s;]+/g
    let match;
    while ((match = re.exec(m[0])) !== null) {
      this.state.shotsManager.delete(match[0])
    }
  } else if (m = /clear\s*--(?<arg>\w+)/g.exec(cmd)) {
    if (m.groups["arg"] == "shots") this.state.shotsManager.clear() // 清除所有镜头
  }
}


class RalEntry{
  typeName = ''
}

class RalDialog{
  /**
   *@constructor 
   * @param {object} json
   * @param {string|null} json.speaker
   * @param {string} json.content 
   */
  constructor(json){
    this.speaker = json.speaker
    this.content = json.content
  }
}

class RalLink{
  /**
   * 
   * @param {object} json 
   * @param {string} json.txt
   * @param {string} json.link
   */
  constructor(json){
    this.txt = json.txt
    this.link = json.link
  }
}

class RalImage{
  /**
   * 
   * @param {object} json 
   * @param {string} json.picPath
   */
  constructor(json){
    this.picPath = json.picPath
  }
}

class RalTitle{
  /**
   * 
   * @param {object} json
   * @param {string} json.title 
   */
  constructor(json){
    this.title = json.title
  }
}
class RalCommand{
  /**
   * 
   * @param {object} json 
   * @param {string} json.cmd
   */
  constructor(json){
    this.cmd = json.cmd
  }
}

export {RalDialog, RalEntry, RalTitle, RalLink, RalCommand, RalImage}