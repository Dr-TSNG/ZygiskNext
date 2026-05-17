export default {
    language: { 
        name: '中文（简体）' 
    },
    main_fragment: {
        dashboard: '状态',
        basic: '基本信息',
        settings: '设置'
    },
    dashboard: {
        root_impl: 'Root 实现',
        zygote_monitor: 'Zygote 监视器',
        zygisk_module_title: 'Zygisk 模块 ({0})',
        zn_module_title: 'ZN 模块 ({0})',
        root_impl_normal: '当前 Root 实现为 {impl}，排除列表将正常工作。',
        root_impl_abnormal: '无法确定 Root 实现，排除列表将不会工作。',
        root_impl_multiple: '当前存在多个 Root 实现，排除列表将不会工作。',
        kernelsu_denylist: 'KernelSU 的排除列表指 App Profile 中被标记为卸载模块的应用。',
        magisk_denylist: 'Magisk 的排除列表指 Magisk 内置的排除列表。',
        apatch_denylist: 'APatch 的排除列表指超级用户中标记为排除模块的应用且未被授予 root 权限。',
    },
    settings: {
        log_to_kernel: '日志写入 dmesg（仅供开发者使用）',
        nonroot_as_denylist: '将非 Root 应用视为排除列表',
        enforce_denylist: '排除列表策略',
        enforce_denylist_desc: '强制：对排除列表中应用，禁止代码注入并还原挂载变更。<br/>仅还原挂载：对排除列表中应用，还原挂载变更但允许代码注入。',
        enforce_denylist_alert: '对普通用户，强烈建议在 KernelSU 管理器中手动关闭内核 umount 功能，以避免配置不当导致挂载点被多次卸载。',
        denylist_disabled: '关闭',
        denylist_enforced: '强制',
        denylist_just_umount: '仅还原挂载',
        anonymous_memory: '使用匿名内存',
        anonymous_memory_desc: '将模块加载到匿名内存。这会破坏日志可读性，但能避免一些过时的检测。',
        zn_linker: '使用 Zygisk Next 链接器',
        zn_linker_desc: '使用内置链接器替代系统链接器加载模块，增强隐蔽性但可能导致兼容性问题。',
    },
    zygote_inject_state: {
        running: '运行中',
        stop_by_user: '停止（用户请求）',
        stop_by_crash: '停止（Zygote 崩溃）',
        running_desc: 'Zygote 监视器正在正常运行。',
        stop_by_user_desc: 'Zygote 监视器被用户停止。',
        stop_by_crash_desc: '检测到 Zygote 反复重启，Zygote 监视器已自动关闭。',
    },
    zygote_state: {
        unknown: '未知',
        injected: '已注入 ({pid})',
        inject_failed: '注入失败 ({pid})',
        skipped: '已跳过 ({pid})',
        unknown_desc: '该 Zygote 的状态未知，可能存在而未启动，或 Zygote 监视器未检测到其启动。',
        injected_desc: '该 Zygote 已被注入 Zygisk ，其进程 ID 为 {pid}',
        inject_failed_desc: '该 Zygote 曾被尝试注入 Zygisk 但失败，其进程 ID 为 {pid}',
        skipped_desc: '该 Zygote 被监视到启动，其进程 ID 为 {pid}，但由于此前系统发生多次软重启，因此停止注入 Zygisk'
    },
    corrupted: {
        title: '模块文件损坏',
        desc: '请还原对 Zygisk Next 的修改后重试'
    },
    module: {
        issue: {
            title: '模块存在问题',
            companion_api_issue: '此模块 {name} 存在 Companion API 使用不当的问题，可能导致进程崩溃和内存泄露，请联系此模块的开发者解决。',
            learn_more: '访问这里了解更多信息：{link}',
            check_banner: '检测到 {0} 个存在问题的模块，请检查模块列表。',
            badge: '存在问题',
        },
        zn: {
            process_count: '{0} 个进程',
        },
    }
}
