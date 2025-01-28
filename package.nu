#!/usr/bin/env nu

def main [
    --out: string = "./"
] {
    let file_name = "save_server.mod.zip"
    let path = $out | path join $file_name

    if ($path | path exists) {
        rm -f $path
    }

    let boot = "./mod/boot.json"
    mut cfg = (open "./mod/boot.template.json")
    $cfg.version = (open "Cargo.toml") | get package.version
    $cfg.scriptFileList = ls "./mod/js" | get name | each { |f| $f | str replace -a "\\" "/" }
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
            # for file in ($cfg | get $key) {
            #     $files = $files | append ("./mod" | path join $file)
            # }
            $files = $files | append ($cfg | get $key)
        }
    }

    print $files

    7z a "-tzip" "-mm=Deflate" "-mx=9" $path ...$files
}