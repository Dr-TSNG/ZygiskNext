export default {
    language: {
        name: 'Español (US)'
    },
    main_fragment: {
        dashboard: 'Estado',
        basic: 'Información básica',
        settings: 'Ajustes'
    },
    dashboard: {
        root_impl: 'Solución de root',
        zygote_monitor: 'Monitor de Zygote',
        zygisk_module_title: 'Sin módulos Zygisk | Módulo Zygisk ({0}) | Módulos Zygisk ({0})',
        zn_module_title: 'Sin módulos ZN | Módulo ZN ({0}) | Módulos ZN ({0})',
        root_impl_normal: 'La solución de root actual es {impl}. La denylist funcionará correctamente.',
        root_impl_abnormal: 'No se pudo determinar la solución de root. La denylist no funcionará.',
        root_impl_multiple: 'Se encontraron múltiples soluciones de root. La denylist no funcionará.',
        kernelsu_denylist: 'La denylist de KernelSU hace referencia a las apps con la opción \'Desmontar módulos\' activada en el apartado Perfil de Aplicación.',
        magisk_denylist: 'La denylist de Magisk hace referencia a su propia lista integrada.',
        apatch_denylist: 'La denylist de APatch hace referencia a las apps con la opción \'Excluir y evitar modificaciones\' activada en el apartado Superusuario.',
    },
    settings: {
        log_to_kernel: 'Escribir en dmesg (para desarrolladores)',
        nonroot_as_denylist: 'Tratar apps sin root como denylist',
        enforce_denylist: 'Política de denylist',
        enforce_denylist_desc: 'Aplicada: Bloquea la inyección de código y revierte modificaciones de montaje para apps en la denylist.<br/><br/>Solo desmontaje: Revierte modificaciones de montaje, pero permite la inyección de código para apps en la denylist.',
        enforce_denylist_alert: 'Para usuarios normales, se recomienda encarecidamente desactivar la función de desmontaje a nivel de kernel en el Manager de KernelSU para evitar que los puntos de montaje se desmonten múltiples veces por una mala configuración.',
        denylist_disabled: 'Desactivada',
        denylist_enforced: 'Aplicada',
        denylist_just_umount: 'Solo desmontaje',
        anonymous_memory: 'Usar memoria anónima',
        anonymous_memory_desc: 'Carga los módulos en memoria anónima. Esto compromete la legibilidad de los logs, pero elude ciertos mecanismos de detección obsoletos.',
        zn_linker: 'Usar linker de Zygisk Next',
        zn_linker_desc: 'Carga los módulos con el linker integrado en lugar del linker del sistema. Esto dificulta la detección pero puede causar problemas de compatibilidad.',
    },
    zygote_inject_state: {
        running: 'En ejecución',
        stop_by_user: 'Detenido por usuario',
        stop_by_crash: 'Detenido por crash',
        running_desc: 'El monitor de Zygote está funcionando correctamente.',
        stop_by_user_desc: 'El monitor de Zygote fue detenido por el usuario.',
        stop_by_crash_desc: 'Se han detectado reinicios continuos de Zygote. El monitor de Zygote se ha detenido automáticamente.',
    },
    zygote_state: {
        unknown: 'Desconocido',
        injected: 'Inyectado ({pid})',
        inject_failed: 'Inyección fallida ({pid})',
        skipped: 'Omitido ({pid})',
        unknown_desc: 'El estado de este Zygote es desconocido; puede que exista, pero que no se haya iniciado, o que el monitor de Zygote no haya detectado su inicio.',
        injected_desc: 'Zygisk ha sido inyectado en este Zygote con PID {pid}.',
        inject_failed_desc: 'Se intentó inyectar Zygisk en este Zygote con PID {pid}, pero la operación falló.',
        skipped_desc: 'Se detectó el inicio de este Zygote con PID {pid}, pero la inyección de Zygisk se omitió debido a múltiples reinicios parciales del sistema.'
    },
    corrupted: {
        title: 'Archivos del módulo corruptos',
        desc: 'Por favor, revierte los cambios en Zygisk Next e inténtalo de nuevo.'
    },
    module: {
        issue: {
            title: 'Este módulo presenta un problema',
            companion_api_issue: 'El módulo {name} utiliza de forma incorrecta la Companion API, lo que podría causar crashes de procesos y fugas de memoria. Por favor, contacta con el desarrollador de este módulo para resolverlo.',
            learn_more: 'Visita este enlace para más información: {link}',
            check_banner: 'No se detectaron módulos con problemas. | Se detectó {0} módulo con problemas. | Se detectaron {0} módulos con problemas.',
            badge: 'Problema',
        },
        zn: {
            process_count: 'Sin procesos | {0} proceso | {0} procesos',
        },
    }
}
