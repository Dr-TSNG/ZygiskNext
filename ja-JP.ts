export default {
    language: { 
        name: '日本語'
    },
    main_fragment: {
        dashboard: 'ダッシュボード',
        basic: '基本情報',
        settings: '設定'
    },
    dashboard: {
        root_impl: 'Root の実装',
        zygote_monitor: 'Zygote の監視',
        modules: 'モジュールなし | モジュール (1) | モジュール ({0})',
        root_impl_normal: 'Root の実装は {impl} です。ブラックリストは適切に機能します。',
        root_impl_abnormal: 'Root の実装を判別できませんでした。ブラックリストは機能しません。',
        root_impl_multiple: '複数の Root の実装を検出したため、ブラックリストは機能しません。',
        kernelsu_denylist: 'KernelSU の DenyList はアプリプロファイルで「モジュールのマウントを解除」としてマークされているアプリを参照します。',
        magisk_denylist: 'Magisk の DenyList は Magisk 内蔵の DenyList を指します。',
        apatch_denylist: 'APatch の DenyList は「除外」が有効化されている状態で Root を無効にしているスーパーユーザーアプリを参照します。',
    },
    settings: {
        log_to_kernel: 'dmesg にログを記録する (開発者のみ)',
        nonroot_as_denylist: '非 Root アプリをブラックリストとして扱う',
        enforce_denylist: 'DenyList ポリシー',
        enforce_denylist_desc: '強制: DenyList 内のアプリに対する変更をすべて元に戻します。<br/>アンマウントのみ: DenyList 内のアプリのマウント変更のみを元に戻します。',
        denylist_disabled: '無効',
        denylist_enforced: '強制',
        denylist_just_umount: 'アンマウントのみ',
        anonymous_memory: '匿名メモリを使用する',
        anonymous_memory_desc: 'モジュールを匿名メモリ上に読み込みます。これによってログの可読性は低下しますが、一部の古い検出メカニズムを回避できます。',
        zn_linker: 'Zygisk Next linker を使用する (実験的)',
        zn_linker_desc: 'モジュールの読み込みにシステムリンカーではなく、内蔵のリンカーを使用します。これによってステルス性が向上しますが、互換性の問題が発生する可能性があります。',
    },
    zygote_inject_state: {
        running: '実行中',
        stop_by_user: 'ユーザーによる停止',
        stop_by_crash: 'Zygote のクラッシュによる停止',
        running_desc: 'Zygote の監視は正常に実行されています。',
        stop_by_user_desc: 'Zygote の監視がユーザーによって停止されました。',
        stop_by_crash_desc: 'Zygote のリピートされた再起動が検出されたため、Zygote の監視を自動で停止しました。',
    },
    zygote_state: {
        unknown: '不明',
        injected: 'インジェクト済み ({pid})',
        inject_failed: 'インジェクトに失敗 ({pid})',
        skipped: 'スキップ済み ({pid})',
        unknown_desc: 'この Zygote のステータスは不明であり、存在はするが開始していないか、または Zygote の監視がその開始を検出しない可能性あります。',
        injected_desc: 'Zygisk が Zygote にインジェクトされました。プロセス ID は {pid} です。',
        inject_failed_desc: 'Zygisk をこの Zygote にインジェクトしようとしましたが失敗しました。プロセス ID は {pid} です。',
        skipped_desc: 'Zygote の起動が監視されました。プロセス ID は {pid} です。ただし、システムの複数回のソフトリブートにより Zygisk のインジェクトは停止されました。'
    },
    corrupted: {
        title: 'モジュールファイルが破損しています',
        desc: 'Zygisk Next の変更を元に戻してから再度お試しください。'
    }
}
