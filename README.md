# DoL Save Server

一个用于适用于`DoL`的Web服务程序，提供在游戏存档时同时在本地创建对应存档文件（需加载配套MOD）

## 使用方法

### 快速使用

1. 下载对应系统的程序压缩包和`save_server.mod.zip`
2. 程序压缩包中的程序和配套MOD(`save_server.mod.zip`)复制到游戏目录
3. 在游戏目录创建或编辑`modList.json`, 并将`save_server.mod.zip`添加到列表最后. 如果包括汉化MOD和图片包，文件内容应如:
    ```json
    [
        "汉化包MOD路径",
        "图片包路径",
        "./save_server.mod.zip"
    ]
    ```
4. 运行程序
5. 打开浏览器, 访问[http://127.0.0.1:5000](http://127.0.0.1:5000)

后续如果存档, 则会生成对应的存档文件在游戏目录的`save`文件夹中.

### 存档文件名格式

```
{save_type}-{save_name}-{slot}.save
```
- `save_type`: 存档类型, 有以下值
    - `new`：表示使用`indexedDB`存档(默认存档方式)
    - `old`: 旧的存档方式
- `save_name`: 创建角色时输入的存档名称
- `slot`: 存档位置(`0`为自动存档)

### 程序运行参数

```
Usage: dol_save_server [OPTIONS]

Options:
      --root <ROOT>          游戏根目录 [default: ./]
      --index <INDEX>        访问"/"时的默认文件名 [default: "Degrees of Lewdity.html"]
      --bind <BIND>          服务地址 [default: 127.0.0.1:5000]
      --save-dir <SAVE_DIR>  存档保存目录 [default: ./save]
  -h, --help                 Print help
  -V, --version              Print version
```