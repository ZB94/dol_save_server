# DoL Save Server

一个用于适用于[Degrees of Lewdity](https://github.com/Eltirosto/Degrees-of-Lewdity-Chinese-Localization)的Web服务程序

也支持[Course of Temptation](https://github.com/BlackTeaPie/Course-of-Temptation-Chinese-Localization), 但需要先修改服务配置文件

已测试适用游戏版本：

- DoL:
  - 0.5.3.x
  - 0.5.4.x
- CoT:
  - 0.6.x
  - 0.7.x

## 主要功能

1. 在游戏存档时同步在服务端创建对应存档文件(需要模组加载器并加载配套模组), 并可随时查看和加载
2. 提供简单的用户认证机制(需手动在配置文件中启用), 启用后需要登录才能开始游戏, 且存档根据用户分开保存

![云存档](./images/云存档.jpg)

## 使用方法

### 快速使用

1. 下载对应系统的压缩包
2. 将压缩包解压到游戏目录
3. 运行程序
4. 打开浏览器, 访问[http://127.0.0.1:5000](http://127.0.0.1:5000)开始游戏

**注意:** `CoT`需要将配置中`index`设置为对应的游戏文件名称(如`CourseOfTemptation-xxx.html`)才能在`/`下直接开始游戏

后续如果存档, 则会生成对应的存档文件在游戏目录的`save`文件夹中.

### 查看存档

- 在游戏存档界面可以查看、加载服务端已保存的存档
- 访问`http://服务地址/save.html`来查看服务端已保存的存档

### 存档文件名格式

```
{save_type}-{save_name}-{slot}.save
```

- `save_type`: 存档类型, 有以下值
  - `00`：表示使用`indexedDB`存档(默认存档方式)
  - `01`: 旧的存档方式
- `save_name`: 创建角色时输入的存档名称
- `slot`: 存档位置(`0`为自动存档)

### 服务配置

运行时如果不存在配置文件(`dol_save_server.toml`), 将会在运行目录生成默认配置文件, **修改配置后需要重启才能生效**.

详细配置说明请查看[`dol_save_server_example.toml`](./dol_save_server_example.toml)

### Docker 部署

1. 下载游戏并保存到当前目录的`game`文件夹中
2. 复制`dol_save_server_docker.toml`并重命名为`dol_save_server.toml`, 并根据需要修改其中的内容
3. 执行指令`docker compose up -d`

默认保存的 `save` 文件夹会自动映射
