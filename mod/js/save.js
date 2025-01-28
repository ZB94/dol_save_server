Save.onSave.add(async function (save, details) {
    function upload(details, n) {
        if (details == null || details.length == 0) {
            return;
        }
        // 获取最新的存档信息
        const last = details.reduce((l, r) => l.data.date > r.data.date ? l : r);
        const data = {
            slot: last.slot,
            name: last.data.metadata.saveName,
            data: Save.serialize(),
            new: n
        };
        // 上传
        if (data.data != null) {
            $.post({
                url: "/api/save",
                data: JSON.stringify(data),
                contentType: "application/json"
            });
        }
    }

    if (details.type === "slot" || details.type === "autosave") {
        // 等待存档结束
        await new Promise(r => setTimeout(r, 1000));
        // 获取当前存档信息并上传
        if (idb.active) {
            // var req = indexedDB.open(idb.dbName);
            // req.onsuccess = _e => {
            //     const db = req.result;
            //     const objStorage = db.transaction("details").objectStore("details");
            //     req = objStorage.getAll();
            //     req.onsuccess = _e => {
            //         const arr = req.result;
            //         if (arr.length == 0) {
            //             return;
            //         }

            //         upload(arr);
            //     };
            // };
            upload(await idb.getSaveDetails(), true);
        } else {
            var details = [];
            let d = Save.get();
            if (d.autosave != null) {
                details.push({ slot: 0, data: d.autosave });
            }
            for (var i = 0; i < d.slots.length; i++) {
                if (d.slots[i] != null) {
                    details.push({ slot: i + 1, data: d.slots[i] });
                }
            }

            upload(details, false);
        }
    }
})
