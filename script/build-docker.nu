def main [
    --push # 是否推送镜像
    --latest(-l) # 是否同时推送为latest
] {
    let tag = open Cargo.toml | get package.version
    let repo = "c113/dol_save_server"
    let image = $"($repo):($tag)"

    docker build --tag $image .
    if $push {
        docker push $image
    }

    if $latest {
        let latest_image = $"($repo):latest"
        docker tag $image $latest_image
        docker push $latest_image
    }
}
