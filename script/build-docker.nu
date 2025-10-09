def main [
    --tag: string
] {
    let tag = if $tag == null {
        open Cargo.toml | get package.version
    } else {
        $tag
    }
    let repo = "c113/dol_save_server"
    let image = $"($repo):($tag)"

    docker build --tag $image .
    docker push $image
}
