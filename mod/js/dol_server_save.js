Save.onSave.add(function (save, details) {
    save = { ...save };
    // save.state = State.marshalForSave(Config.history.maxSessionStates, true);
    save.state = State.marshalForSave();

    if (details.type === "slot" || details.type === "autosave") {
        new Promise(r => setTimeout(r, 1))
            .then(async function () {
                if (save.state.history) {
                    save.state.delta = State.deltaEncode(save.state.history);
                    delete save.state.history;
                }

                let details = [];
                // 获取当前存档信息
                if (idb.active) {
                    details = await idb.getSaveDetails();
                } else {
                    let d = Save.get();
                    if (d.autosave != null) {
                        details.push({ slot: 0, data: d.autosave });
                    }
                    for (var i = 0; i < d.slots.length; i++) {
                        if (d.slots[i] != null) {
                            details.push({ slot: i + 1, data: d.slots[i] });
                        }
                    }
                }

                if (details == null || details.length == 0) {
                    return;
                }
                // 获取最新的存档信息
                const last = details.reduce((l, r) => l.data.date > r.data.date ? l : r);

                const data = {
                    slot: last.slot,
                    name: V.saveName || (V.saveId).toString(),
                    save: JSON.stringify(save),
                    story: Story.domId,
                    new: idb.active
                };
                // 上传
                let dss_server = localStorage.getItem("dss_server") || "";
                fetch(`${dss_server}/api/save`, {
                    credentials: "include",
                    method: "POST",
                    body: JSON.stringify(data),
                    headers: {
                        "Content-Type": "application/json",
                    },
                })
                    .then(async resp => {
                        if (!resp.ok) {
                            throw new Error(await resp.json());
                        }
                    })
                    .catch(error => {
                        console.log(error);
                        window.alert(`云存档上传失败`);
                    });
            });
    }
})


let dss_server = localStorage.getItem("dss_server") || "";

// 判断登录状态是否正常
fetch(`${dss_server}/api/alive`, { credentials: "include" })
    .then(resp => {
        console.log("alive", resp);
        if (!resp.ok) {
            window.alert("云存档功能需要登录才能正常使用, 请在存档界面的云存档处登录");
        }
    })
    .catch(error => {
        window.alert("云存档请求异常, 请在存档界面的云存档处设置服务器地址");
    });


// PWA
fetch(`${dss_server}/api/pwa/enabled`, { credentials: "include" })
    .then(async resp => {
        const enabled = await resp.json();
        if (enabled) {
            $('<link crossorigin="use-credentials" rel="manifest" href="/pwa/manifest.json">').appendTo("head");
            $('<script>if (typeof navigator.serviceWorker !== "undefined") { navigator.serviceWorker.register("/sw.js"); }</script>').appendTo("body");
        }
    });
