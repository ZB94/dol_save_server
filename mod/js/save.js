Save.onSave.add(function (save, details) {
    if (details.type === "slot" || details.type === "autosave") {
        var req = indexedDB.open("degrees-of-lewdity");
        req.onsuccess = _e => {
            const db = req.result;
            const objStorage = db.transaction("details").objectStore("details");
            req = objStorage.getAll();
            req.onsuccess = _e => {
                const arr = req.result;
                if (arr.length == 0) {
                    return;
                }

                const last = arr.reduce((l, r) => l.data.date > r.data.date ? l : r);

                const data = {
                    slot: last.slot,
                    name: last.data.metadata.saveName,
                    data: Save.serialize(),
                };

                if (data.data != null) {
                    $.post({
                        url: "/api/save",
                        data: JSON.stringify(data),
                        contentType: "application/json"
                    });
                }
            };
        };
    }
})