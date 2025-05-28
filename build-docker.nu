def main [] {
    let tag = open Cargo.toml | get package.version
    let repo = "c113/dol_save_server"
    let image = $"($repo):($tag)"

    docker build --tag $image .
    docker push $image
}
