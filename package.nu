#!/usr/bin/env nu

def main [
    --out: string = "./"
] {
    let file_name = "save_server.mod.zip"
    let path = $out | path join $file_name

    if ($path | path exists) {
        rm -f $path
    }

    conv_savelist

    let boot = "./mod/boot.json"
    mut cfg = (open "./mod/boot.template.json")
    $cfg.version = (open "Cargo.toml") | get package.version
    $cfg.scriptFileList = ls "./mod/js" | get name | each { |f| $f | str replace -a "\\" "/" }
    $cfg.tweeFileList = ls "./mod/twee" | get name | each { |f| $f | str replace -a "\\" "/" }
    $cfg.additionFile = ["README.md", "LICENSE"]

    $cfg | save -f $boot

    mut files = [$boot]

    for key in [
        "styleFileList",
        "scriptFileList",
        "tweeFileList",
        "imgFileList",
        "additionFile",
        "scriptFileList_inject_early",
        "scriptFileList_earlyload",
        "scriptFileList_preload",
        "additionBinaryFile",
        "additionDir"
    ] {
        if $key in $cfg {
            $files = $files | append ($cfg | get $key)
        }
    }

    print $files

    7z a "-tzip" "-mm=Deflate" "-mx=9" $path ...$files
}

def conv_savelist [] {
    mkdir mod/twee
    open web/save.html |
    lines |
    skip until { |l| $l =~ "<body>" } |
    take until { |l| $l =~ "</html>" }  |
    insert 0 ":: dss_save_list [widget nobr]\n" |
    str join "\n" |
    str replace '<body>' '<<widget "dss_save_list">>' |
    str replace '</body>' '<</widget>>' |
    str replace -r '<script.*?>' '<<script>>' |
    str replace '</script>' '<</script>>' |
    str replace -r -a '<!-- (.*?) -->' '$1' |
    str replace -r -a '//\s+' '' |
    save -f mod/twee/dss_save_list.twee
}
