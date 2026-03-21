export default {
    language: { 
        name: '中文（繁體）' 
    },
    main_fragment: {
        dashboard: '儀表板',
        basic: '基本訊息',
        settings: '設定'
    },
    dashboard: {
        root_impl: 'Root 實現',
        zygote_monitor: 'Zygote 監視器',
        modules: '模組 ({0})',
        root_impl_normal: '目前 Root 實現為 {impl}，排除列表將正常工作。',
        root_impl_abnormal: '無法確定 Root 實現，排除列表將不會工作。',
        root_impl_multiple: '目前存在多個 Root 實現，排除列表將不會工作。',
        kernelsu_denylist: 'KernelSU 的排除列表指 App Profile 中被標記為解除安裝模組的應用。',
        magisk_denylist: 'Magisk 的排除列表指 Magisk 內建的排除列表。',
        apatch_denylist: 'APatch 的排除列表指超級使用者中標記為排除模組的應用且未被授予 root 許可權。',
    },
    settings: {
        log_to_kernel: '日誌寫入 dmesg（僅供開發者使用）',
        nonroot_as_denylist: '將非 root 應用視為排除列表',
        enforce_denylist: '排除列表策略',
        enforce_denylist_desc: '強制：還原對位於排除列表中的 App 做出的所有變更。<br/>僅還原掛載：僅還原對位於排除列表中的 App 做出的掛載變更。',
        denylist_disabled: '關閉',
        denylist_enforced: '強制',
        denylist_just_umount: '僅還原掛載',
        anonymous_memory: '使用匿名記憶體',
        anonymous_memory_desc: '將模組載入到匿名記憶體。這會破壞日誌可讀性，但能避免一些過時的偵測。',
        zn_linker: '禁用 Zygisk Next 連結器',
        zn_linker_desc: '關閉內建連結器，改用系統連結器載入模組。可能降低隱蔽性，但提升相容性。',
    },
    zygote_inject_state: {
        running: '執行中',
        stop_by_user: '停止（使用者請求）',
        stop_by_crash: '停止（Zygote 崩潰）',
        running_desc: 'Zygote 監視器正在正常執行。',
        stop_by_user_desc: 'Zygote 監視器被使用者停止。',
        stop_by_crash_desc: '檢測到 Zygote 反覆重啟，Zygote 監視器已自動關閉。',
    },
    zygote_state: {
        unknown: '未知',
        injected: '已注入 ({pid})',
        inject_failed: '注入失敗 ({pid})',
        skipped: '已跳過 ({pid})',
        unknown_desc: '該 Zygote 的狀態未知，可能存在而未啟動，或 Zygote 監視器未檢測到其啟動。',
        injected_desc: '該 Zygote 已被注入 Zygisk ，其程序 ID 為 {pid}',
        inject_failed_desc: '該 Zygote 曾被嘗試注入 Zygisk 但失敗，其程序 ID 為 {pid}',
        skipped_desc: '該 Zygote 被監視到啟動，其程序 ID 為 {pid}，但由於此前系統發生多次軟重啟，因此停止注入 Zygisk'
    },
    corrupted: {
        title: '模組檔案損壞',
        desc: '請還原對 Zygisk Next 的修改後重試'
    }
}
