export default {
    language: { 
        name: 'Russian (RU)'
    },
    main_fragment: {
        dashboard: 'Панель инструментов',
        basic: 'Основная информация',
        settings: 'Настройки'
    },
    dashboard: {
        root_impl: 'Источник рут',
        zygote_monitor: 'Мониторинг Zygote',
        modules: 'Нет модулей | Модуль (1) | Модули ({0})',
        root_impl_normal: 'Текущий источник рут - {impl}, denylist будет работать правильно.',
        root_impl_abnormal: 'Не удалось определить источник рут, denylist не будет работать.',
        root_impl_multiple: 'Обнаружено несколько источников рут, denylist не будет работать.',
        kernelsu_denylist: 'Denylist в KernelSU применяется к приложениям для которых включена опция \'Размонтировать модули\' в App Profile.',
        magisk_denylist: 'Denylist в Magisk применяется к встроенному в Magisk\'s denylist.',
        apatch_denylist: 'Denylist в APatch применяется к приложениям которые отмечены на вкладке SuperUser опцией \'Исключить\'',
    },
    settings: {
        log_to_kernel: 'Логи в dmesg (только для разработчиков)',
        enforce_denylist: 'Политика Denylist',
        enforce_denylist_desc: 'Принудительно: любые изменения для приложений в списке запрещенных будут отменены.<br/>Только размонтирование: будут отменены изменения для приложений из denylist.',
        denylist_disabled: 'Отключен',
        denylist_enforced: 'Принудительно',
        denylist_just_umount: 'Только размонтирование',
        anonymous_memory: 'Использовать анонимную память',
        anonymous_memory_desc: 'Загрузка модулей в анонимную память. Это ухудшает читаемость журнала, но позволяет обойти некоторые устаревшие механизмы обнаружения.',
        zn_linker: 'Использовать линкер Zygisk Next (экспериментально)',
        zn_linker_desc: 'Использовать встроенный компоновщик вместо системного для загрузки модулей. Это повысит скрытность, но может вызвать проблемы совместимости.',
    },
    zygote_inject_state: {
        running: 'Запущен',
        stop_by_user: 'Остановлен пользователем',
        stop_by_crash: 'Остановлен ошибкой zygote',
        running_desc: 'Мониторинг Zygote работает нормально.',
        stop_by_user_desc: 'Мониторинг Zygote остановлен пользователем.',
        stop_by_crash_desc: 'Обнаружены повторные перезапуски Zygote, мониторинг Zygote автоматически остановлен.',
    },
    zygote_state: {
        unknown: 'Неизвестный',
        injected: 'Встроенно ({pid})',
        inject_failed: 'Ошибка встраивания ({pid})',
        skipped: 'Пропущено ({pid})',
        unknown_desc: 'Статус этого Zygote неизвестен и может работать, но не быть запущенн или мониторинг Zygote не может обнаружить его запуск.',
        injected_desc: 'Zygisk был внедрён в Zygote. Идентификатор его процесса: {pid}',
        inject_failed_desc: 'Попытка внедрить Zygisk в Zygote не удалась. Идентификатор процесса: {pid}',
        skipped_desc: 'Zygote был отслежен до начала работы. Идентификатор процесса: {pid}. Внедрение Zygisk было остановлено из-за многочисленных перезагрузок системы.'
    },
    corrupted: {
        title: 'Файлы модуля повреждены',
        desc: 'Пожалуйста, верните Zygisk Next к начальным настройкам и повторите попытку.'
    }
}
