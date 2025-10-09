
def main [
    config?: string = "./download.toml" # 下载配置
] {
    mut cfg = open $config

    let proxy = if "download_proxy" in $cfg {
        $cfg.download_proxy
    } else {
        null
    }

    let name = download $cfg.repo $cfg.pattern "." $proxy $cfg.filter?
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
        let mod_dir = $cfg.dir | path join "mod"
        mkdir $mod_dir

        $cfg.mods | each { |mod|
            let name = download $mod.repo $mod.pattern $mod_dir $proxy $mod.filter?
            $"mod/($name)"
        } | to json | save -f ($cfg.dir | path join "modList.json")
    }

    print 全部下载完成
}


def download [
    repo: string # github 项目
    pattern: string # 要下载的文件 正则表达式
    outdir: path # 输出目录
    proxy? # 下载代理
    filter? # 获取release的筛选条件 作为`gh release list`指令的`--jq`参数 默认为获取最新一个release
] {
    print $"准备下载 repo=($repo) pattern=($pattern)"

    let filter = if $filter == null {
        'map(select(.isLatest))'
    } else {
        $filter
    }
    # 获取 release 的 tag
    let fields = gh release list --help | lines | skip until { |l| $l == "JSON FIELDS" } | skip 1 | first | str trim | split row ', ' | str join ','
    let tag = (gh release list -R $repo --json $fields -q $filter | from json | first | get tagName)

    # 获取满足条件文件
    let assets = (gh release view -R $repo --json assets $tag | from json | get assets | where name =~ $pattern)

    # 下载
    if ($assets | length) == 1 {
        let asset = $assets | first
        let name = $asset.name
        print $"开始下载 ($name)"
        # gh release download ...($names | each { |name| ["-p", $name] } | flatten) -D $outdir -R $repo --clobber $tag
        if $proxy == null {
            gh release download -p $name -D $outdir -R $repo --clobber $tag
        } else {
            let url = $"($proxy)/($asset.url)"
            curl -o $"($outdir | path join $name)" $url
        }

        print "下载完成"
        return $name
    } else if ($assets | length) > 1 {
        print -e "满足条件的文件数量超过 1"
        print -e ($assets | get name)
        exit 1
    } else {
        print -e "未找到满足条件的文件"
        exit 1
    }
}
