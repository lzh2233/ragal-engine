# RAGAL engine-v0.5.3

> **🎉🎉🎉RAGAL大更新啦！🥳🥳🥳**

## 快速了解

- 版本号: v0.5.3
- 新增功能
  - [工程化结构(ragal.cfg.yml配置文件)](#ragal配置对象)
  - [创建初始化的工程](#新建初始化的工程)
  - [打包工程](#工程打包)
  - [开发时可以打开web的开发工具](#打开开发工具)
  - [支持自定义文本本地变量和工程的全局变量](#变量的定义)
  - [文中可嵌入*Javascript*脚本并即时运行](#嵌入javascript)
  - [可自定义标题页面](#自定义标题页)
  - [支持条件式分支选项](#条件式分支)
  - [立绘状态的支持](#立绘状态)
- 平台支持: win7/10/11
- 标签: `markdown式文件所写及所得`, `可嵌入js脚本`
- 二次开发所需技术: `javascript-css-html`, `markdown语法`,`美术`

##  v0.5.3的新增内容指南

0.5版本真的改了很多，我还记得v0.1版时单纯的解析md，到v0.2版本的初具工程化，再到v0.3启动器与ui引导界面的分离，到这一版的可以打包工程。经历很多的重构（主要是技术与经验不够，走了很多歪路，至今都不知埋了多少雷了🎃）。这里先申明几个概念：

1. 启动器是播放gal的程序，我后面称其为galplayer
2. ui引导界面是集成了打开gal工程，创建工程，打包工程的程序，后称其为ragal

### ragal配置对象

工程化结构的核心就是基于一个配置文件中的基本信息，包括了启动游戏时的屏幕大小，标题，关键的资源文件目录的路径等设置；这些关键信息都可记录在`y(a)ml`文件中，ragal打开工程是通过打开`yml`并读取其中的关键内容来加载并启动galplayer的。工程的根目录就是此yml配置文件所在目录路径。当然，yml配置文件需要一定的结构。

#### 配置结构

符合ragal配置格式的yml文件称为**ragal配置对象**，您可以通过修改它来自定义gal工程。下面我们默认配置文件名为`ragal.cfg.yml`。ragal配置对象大致有以下对象构成：

- [base](#BaseConfig)
- [src](#PathConfig)
- [game](#GameConfig)

```yaml
#Example ragal.cfg.yml file

base:
  title: demo
  title_page: /path/cfg/yml
  theme: $theme.css
  icon: icon.ico
  mainresolution: [1080, 720]

src:
  entry: src/entry.md
  img: img
  standing: standing
  audio: audio
  custom_script: custom_script
  custom_css: []

game:
  typing:
    speed: 4
    size: 2.9
  standing: 
    1:
      - [30%, -10%, 100%]
    2: 
      - [10%, -10%, 100%]
      - [50%, -10%, 100%]
  volume:
    bgm: 100
    se: 100
```



##### BaseConfig

| Name             | Type       | Default | Description          |
| :--------------- | ---------- | ------- | -------------------- |
| *title*          | String     | 必填    | gal标题              |
| *title_page*     | String?    | null    | 自定义标题页html路径 |
| *theme*          | String?    | null    | 主题相关的css路径    |
| *icon*           | String?    | null    | 图标路径             |
| *mainresolution* | [Number;2] | 必填    | 窗口尺寸[宽x高]      |

目前`theme`，`icon`属性无关紧要

##### PathConfig

| Name          | Type   | Default | Description           |
| ------------- | ------ | ------- | --------------------- |
| *entry*       | String | 必填    | 剧本入口文件路径      |
| *img*         | String | 必填    | 图片目录路径          |
| *standing*    | String | 必填    | 立绘目录路径          |
| *audio*       | String | null    | 声音目录路径          |
| custom_script | String | null    | 脚本目录路径          |
| *custom_css*  | Vec    | []      | 自定义css文件路径集合 |

目前，`audio`，`custom_css`属性无关紧要

##### GameConfig

| Name       | Type   | Default               | Description                             |
| ---------- | ------ | --------------------- | --------------------------------------- |
| *typing*   | Object | {speed: 4, size: 2.9} | 对话的打字速度和字体大小默认值          |
| *standing* | Object | null                  | 默认立绘屏幕站位，只支持1~2人的人物镜头 |
| *volume*   | Object | {bgm: 100, se: 100}   | 音频默认值                              |

目前`typing`属性数值是锁死的（我感觉当前挺不错的，后续会改的）

目前`volume`属性无关紧要（毕竟还没实现音频模块）

### 新建初始化的工程

这个简单😆直接点击ragal中的`build project`,选中目标目录，会在其下创建ragal的配置文件及其相关内容

### 工程打包

打包工程是v0.4.0中的ragal新增的功能，点击`bundle project`，需要选择两个路径：

1. 需要打包工程的ragal配置对象所在路径
2. 需要打包到的目标目录路径

注意事项：角色立绘只应该放置在standing文件夹内，并其alias.json只应该为相对路径

### 打开开发工具

在ragal打开工程时，`ctrl+shift+i`组合键可打开galplayer的开发工具，供二次开发的调试

### 变量的定义

用户可以在galplayer运行时定义自己的变量，分为全局的变量和剧本文本的本地变量。

#### 全局变量

运行时的全局变量需定义在`init.js`文件中，并且要在ragal.cfg.yml同级目录下。对于目前版本，没有特别的变量api，以后可能考虑。

注意，全局变量是游戏运行时才会被加载进游戏中，标题页是没有全局变量的。

在嵌入式js中，有`global`变量挂载了所定义的全局变量

#### 局部变量

在剧本头部可定义局部变量，如下所示：

```markdown
---
color: red
flag: false
---
```

通过this来访问这些本地变量



### 嵌入javascript

galplayer是支持剧本中插入js脚本并即时运行，并每个脚本块间都是相互隔离的，但都能访问本剧本的局部变量`this`；剧本间也是相互隔离的，但`global`对象是全局的，所有剧本共享，以下是样例：

~~~markdown
---
hp: 100
mp: 100
atk:10
---
<!-- 本地变量 -->


我：你休想！

怪物对我发动了攻击！

```js
let monster = global.monster	//global.state就是custom_state.json文件中描述的对象
this.hp -= monster.atk
console.log(dirname)
if (this.hp < 0) shelf.chipIn(null, "菜就多练！打不赢就别玩😅") //会打印到控制台的，可以按ctrl+shift+i来打开控制台查看效果
```

我：✌🙄✌
哒咩！
~~~

注意，嵌入式js的外部api被挂载在`shelf`变量之中。`dirname`是当前md的目录，方便相对引用资源。

### 自定义标题页

`title_page`中定义了标题页的所在路径，如果你有能力可以自己写个漂亮的页面。另外，重要的是，目前版本有三个api可供调用：

- onStart()    开始游戏
- onExit()      退出游戏（退出程序）

你可以在你的html里使用它们

### 条件式分支

可以根据条件是否满足来实现选项的出现，从而实现复杂的分支剧情，这不是很有趣吗！

条件分支根据选项语法的扩展，目前只支持一层，条件间是取或的，取第一个为真的条件，下面是个例子

```markdown
- [我去](other-end.md)
	- this.hp > 0		
	- this.atk > 0
- 我不去
<!-- hp和atk其中一个大于0就能展示【我去】的选项 -->

- [我去](other-end.md)
	- this.hp > 0 && this.atk > 0
- 我不去
<!-- hp和atk都必须大于0才能展示【我去】的选项 -->
```

#### advance

这里有更加进阶的写法，能方便你写出更加灵活复杂的剧本。

- 当条件满足时的立即执行函数
- 点击选项时的回调函数

**条件满足时的立即执行函数**

```markdown
- [我去](other-end.md)
	- [this.hp > 0](badend.js)	<!-- 满足条件时可以执行里面的暴露的默认函数 -->
	- this.atk > 0	<!-- 进入默认的剧本 -->
- 我不去
```

这里我们详细讲解`badend.js`的样例，而这里最为重要的是需要暴露默认函数，`this`指针指向本剧本的state，这个函数可以返回一个`object{url, txt, opt}`，这个返回值修改本选项，以下是个**example**:

```js
export default function(){
    if (this.atk > 0){
        return {
            txt: "会赢的",
            url: "goodend.md",
            opt: "callback"
        }
    }
    return {
        txt: "牢大别肘",
        url: "seeYouAgain.md",
        opt: "switch"
    }
}
```

你发现了，上面有个很有趣的`opt`成员变量，`opt`取决了怎么切换剧本的方式，其值有以下枚举：

- switch 直接切换
- callback 回调剧本（像是插入）
- custom 点击时调用自定义的`js`脚本

这里有很快捷的在markdown中的写法：

```markdow
- [会赢的 #callback](end1.md) 
- [让他们瞧瞧我们的厉害](end2.md)	<!-- 默认switch -->
- [战斗，爽！ #custom](end3.js)
```

`#custom`是有十分强大的功能，这依赖于ragal向其提供了控制剧本流的api：

```js
export default function(dirname, {switchMD, chipIn, callbackMD}){
	if(this.a > 0) return switchMD(`${dirname}/end1.md`)
    chipIn(null, "插入一句话")	//这个api我很不满意，因为这个是插入对话是先进后出的，很反直觉，之后会改的
    return callbackMD(`${dirname}/end2.md`)
    // 注意，使用switchMD & callbackMD 这俩api时，要返回其结果，因为这俩是异步api
}
```



### 立绘状态

角色的状态有立绘差分了，写起来更符合直觉😋

#### alias.json中的立绘资源映射写法

```json
{
    "我": "/path/img",
    "小熊": {
        "default" : "/path/默认的立绘",
        "闷闷不乐" : "/path/小熊-闷闷不乐.png"
    }
}
```

#### 剧本markdown中的写法

```markdown
我：你好！

小熊（闷闷不乐）：。。。啊，早上好。

小熊看到你连忙装作无事。

小熊：昨天睡得还好吗？	<!-- 默认立绘 -->
```

<br/>

### 杂项

```markdown
<!-- 画外音 -->
<!-- 
	以下两种是有区别的！
	os是不会重置当前的镜头的
-->
os: 他已经无路可退了

他已经无路可退了


<!-- 一人多句话的语法糖 -->
我：我绝不去那里，我太菜了。
再说，那儿也不会闹出什么事，去了也没用。
所以，我劝你也别去，要是真闹出事来你就脱不了身了。
<!-- 等同于以下写法 -->
我：我绝不去那里，我太菜了。

我：再说，那儿也不会闹出什么事，去了也没用。

我：所以，我劝你也别去，要是真闹出事来你就脱不了身了。
```

其他的设置背景，设置镜头等语法可参考上一版ragal指南，不过对于镜头的默认位置设置是支持的，可看[ragal配置对象.game.standing](#GameConfig)，可以这样方便的设置镜头了`> +{我,小熊}`

## Future

下一版想要什么功能，可以向我提提**issue**🙂