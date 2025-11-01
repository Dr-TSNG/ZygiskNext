export default {
    language: { 
        name: '中文（香港）' 
    },
    main_fragment: {
        dashboard: '儀表板',
        basic: '基本資訊',
        settings: '設定'
    },
    dashboard: {
        root_impl: 'Root 實現方案',
        zygote_monitor: 'Zygote 監視器',
        modules: '模組 ({0})',
        root_impl_normal: '目前 Root 實現方案為 {impl}，黑名單將正常運作。',
        root_impl_abnormal: '無法判斷 Root 實現方案，黑名單將無法運作。',
        root_impl_multiple: '目前偵測到多個 Root 實現方案，黑名單將無法運作。',
        kernelsu_denylist: 'KernelSU 的黑名單指在 App Profile 中被標記為解除安裝模組的應用程式。',
        magisk_denylist: 'Magisk 的黑名單指在 Magisk 內置的黑名單。',
        apatch_denylist: 'APatch 的黑名單指在超級使用者中標記為排除模組的應用程式且未被授予 root 權限。',
    },
    settings: {
        log_to_kernel: '記錄檔寫入 dmesg（僅供開發人員使用）',
        nonroot_as_denylist: '將非 Root 應用程式視作為黑名單',
        enforce_denylist: '黑名單方案',
        enforce_denylist_desc: '強制：重設對位於黑名單中的應用程式所做的所有變更。<br/>僅重設掛載：僅重設對位於黑名單中的應用程式所做的掛載變更。',
        denylist_disabled: '關閉',
        denylist_enforced: '強制',
        denylist_just_umount: '僅重設掛載',
        anonymous_memory: '使用匿名記憶體',
        anonymous_memory_desc: '將模組載入到匿名記憶體。這會破壞記錄檔可讀性，但能避免一些過時的偵測。',
        zn_linker: '使用 Zygisk Next 連結器（實驗性）',
        zn_linker_desc: '使用內置連結器取代系統連結器載入模組，增強隱蔽性但可能導致兼容性問題。',
    },
    zygote_inject_state: {
        running: '運作中',
        stop_by_user: '停止（使用者請求）',
        stop_by_crash: '停止（Zygote 崩潰）',
        running_desc: 'Zygote 監視器正在正常運作。',
        stop_by_user_desc: 'Zygote 監視器被使用者強制停止。',
        stop_by_crash_desc: '偵測到 Zygote 反覆重新啟動，Zygote 監視器已自動關閉。',
    },
    zygote_state: {
        unknown: '未知',
        injected: '已注入 ({pid})',
        inject_failed: '注入失敗 ({pid})',
        skipped: '已跳過 ({pid})',
        unknown_desc: '該 Zygote 的狀態未知，可能存在而未啟動，或 Zygote 監視器未偵測到其啟動。',
        injected_desc: '該 Zygote 已被注入 Zygisk ，其進程 ID 為 {pid}',
        inject_failed_desc: '該 Zygote 曾被嘗試注入 Zygisk 但失敗，其進程 ID 為 {pid}',
        skipped_desc: '該 Zygote 被監視到啟動，其進程 ID 為 {pid}，但由於此前系統發生多次軟重啟，因而停止注入 Zygisk'
    },
    corrupted: {
        title: '模組檔案損毀',
        desc: '請重設對 Zygisk Next 的修改後重試'
    }
}
