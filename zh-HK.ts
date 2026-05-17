export default {
    language: { 
        name: '中文（香港）' 
    },
    main_fragment: {
        dashboard: '狀態',
        basic: '基本資訊',
        settings: '設定'
    },
    dashboard: {
        root_impl: 'Root 方案',
        zygote_monitor: 'Zygote 監測器',
        zygisk_module_title: 'Zygisk 模組 ({0})',
        zn_module_title: 'ZN 模組 ({0})',
        root_impl_normal: '目前 Root 方案為 {impl}，排除清單運作正常。',
        root_impl_abnormal: '無法確定 Root 方案，排除清單無法運作。',
        root_impl_multiple: '偵測到多個 Root 方案，排除清單無法運作。',
        kernelsu_denylist: 'KernelSU 排除清單係 App Profile 標記為「卸載模組」嘅程式。',
        magisk_denylist: 'Magisk 排除清單係 Magisk 內置嘅排除清單。',
        apatch_denylist: 'APatch 排除清單係標記為「排除模組」且未獲 Root 權限嘅程式。',
    },
    settings: {
        log_to_kernel: '將紀錄檔寫入 dmesg（僅供開發者使用）',
        nonroot_as_denylist: '將非 Root 應用程式視為排除清單',
        enforce_denylist: '排除清單策略',
        enforce_denylist_desc: '強制：針對排除清單內嘅 App，禁止代碼注入並還原掛載變更。<br/>僅還原掛載：針對排除清單內嘅 App，還原掛載變更但允許代碼注入。',
        enforce_denylist_alert: '對於一般用戶，強烈建議喺 KernelSU 管理器中手動關閉核心 umount 功能，以免因設定不當導致掛載點被多次卸載。',
        denylist_disabled: '關閉',
        denylist_enforced: '強制',
        denylist_just_umount: '僅還原掛載',
        anonymous_memory: '使用匿名記憶體',
        anonymous_memory_desc: '將模組載入至匿名記憶體。雖然會令紀錄檔難以閱讀，但可以避開部分過時嘅偵測手段。',
        zn_linker: '使用 Zygisk Next 連結器',
        zn_linker_desc: '使用內置連結器取代系統連結器載入模組，增強隱蔽性但可能導致兼容性問題。',
    },
    zygote_inject_state: {
        running: '運作中',
        stop_by_user: '停止（用戶要求）',
        stop_by_crash: '停止（Zygote 崩潰）',
        running_desc: 'Zygote 監測器正常運作。',
        stop_by_user_desc: 'Zygote 監測器被用戶停止。',
        stop_by_crash_desc: '偵測到 Zygote 反覆重啟，Zygote 監測器已自動關閉。',
    },
    zygote_state: {
        unknown: '未知',
        injected: '已注入 ({pid})',
        inject_failed: '注入失敗 ({pid})',
        skipped: '已跳過 ({pid})',
        unknown_desc: '該 Zygote 狀態未知，可能存在但未啟動，或監測器未偵測到其啟動。',
        injected_desc: '該 Zygote 已被注入 Zygisk，程序 ID 為 {pid}',
        inject_failed_desc: '曾嘗試向該 Zygote 注入 Zygisk 但失敗，程序 ID 為 {pid}',
        skipped_desc: '監測到該 Zygote 啟動，程序 ID 為 {pid}，但由於系統早前發生多次軟重啟，已停止注入 Zygisk'
    },
    corrupted: {
        title: '模組檔案損毀',
        desc: '請還原對 Zygisk Next 嘅修改後再試'
    },
    module: {
        issue: {
            title: '模組存在問題',
            companion_api_issue: '此模組 {name} 存在 Companion API 使用不當嘅問題，可能導致行程崩潰同記憶體洩漏，請聯絡此模組嘅開發者解決。',
            learn_more: '訪問呢度了解更多信息：{link}',
            check_banner: '檢測到 {0} 個有問題嘅模組，請檢查模組列表。',
            badge: '有問題',
        },
        zn: {
            process_count: '{0} 個進程',
        },
    }
}