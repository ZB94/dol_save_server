
def main [
    --config: string = "./download.toml" # 下载配置
] {
    let cfg = open $config

    let name = download $cfg.repo $cfg.pattern "."
    let dir = $cfg.dir | path basename
    let parent = $cfg.dir | path dirname
    let backup = $parent | path join $"bak_($dir)"

    if ($backup | path exists) {
        rm -rf $backup
        print 已删除已有备份文件
    }

    if ($cfg.dir | path exists) {
        mv $cfg.dir $backup
        print $"已将现有文件移动到 ($backup)"
    }

    mkdir $cfg.dir
    7z e $"-o($cfg.dir)" $name
    rm -f $name

    if "mods" in $cfg {
        let mod_dir = $cfg.dir | path join "mods"
        mkdir mod_dir

        $cfg.mods | each { |mod|
            let name = download $mod.repo $mod.pattern $mod_dir
            $"mods/($name)"
        } | to json | save -f ($cfg.dir | path join "ModList.json")
    }

    print 全部下载完成
}


def download [
    repo: string # github 项目
    pattern: string # 要下载的文件 正则表达式
    outdir: path # 输出目录
] {
    print $"准备下载 repo=($repo) pattern=($pattern)"

    # 获取最后一个 release 的 tag
    let tag = (gh release list -R $repo -L 1 --json tagName | from json | first | get tagName)

    # 获取满足条件文件
    let names = (gh release view -R $repo --json assets $tag | from json | get assets | where name =~ $pattern | get name)

    # 下载
    if ($names | length) == 1 {
        let name = $names | first
        print $"开始下载 ($name)"
        # gh release download ...($names | each { |name| ["-p", $name] } | flatten) -D $outdir -R $repo --clobber $tag
        gh release download -p $name -D $outdir -R $repo --clobber $tag
        print "下载完成"

        return $name
    } else if ($names | length) > 1 {
        print -e "满足条件的文件数量超过 1"
        print -e $names
        exit 1
    } else {
        print -e "未找到满足条件的文件"
        exit 1
    }
}
