# DoL Save Server

一个用于适用于`DoL`的Web服务程序，提供在游戏存档时同时在本地创建对应存档文件（需加载配套MOD）

## 使用方法

### 快速使用

1. 下载对应系统的压缩包
2. 将压缩包解压到游戏目录
3. 运行程序
4. 打开浏览器, 访问[http://127.0.0.1:5000](http://127.0.0.1:5000)开始游戏

后续如果存档, 则会生成对应的存档文件在游戏目录的`save`文件夹中.

### 查看存档

可以通过访问`http://服务地址/saves`来查看服务端已保存的存档

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
      --root <ROOT>
          游戏根目录

          [default: ./]

      --index <INDEX>
          访问"/"时的默认文件名

          [default: "Degrees of Lewdity.html"]

      --bind <BIND>
          服务地址

          [default: 127.0.0.1:5000]

      --save-dir <SAVE_DIR>
          存档保存目录

          [default: ./save]

      --no-init-mod
          启动时跳过初始化模组流程

      --enable-auth
          是否启用登录验证

      --auth-file <AUTH_FILE>
          用户列表文件路径

          文件格式应如: { "用户名1": "密码1", "用户名2": "密码2", ... }

          注意: 用户名应为运行系统的合法目录路径, 否则可能导致存档保存失败

          [default: ./auth.json]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```