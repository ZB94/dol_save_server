Save.onSave.add(function (save, details) {
    // 避免存档属性在其他地方被删除
    save = { ...save };

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
                    name: last.data.metadata.saveName,
                    save: JSON.stringify(save),
                    story: Story.domId,
                    new: idb.active
                };
                // 上传
                $.post({
                    url: "/api/save",
                    data: JSON.stringify(data),
                    contentType: "application/json",
                    error: () => {
                        window.alert("存档上传失败");
                    }
                });
            });
    }
})

// PWA
fetch("/api/pwa/enabled")
    .then(async resp => {
        const enabled = await resp.json();
        if (enabled) {
            $('<link crossorigin="use-credentials" rel="manifest" href="/manifest.json">').appendTo("head");
            $('<script>if (typeof navigator.serviceWorker !== "undefined") { navigator.serviceWorker.register("/sw.js"); }</script>').appendTo("body");
        }
    })


