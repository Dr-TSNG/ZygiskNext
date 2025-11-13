export default {
    language: { 
        name: 'Français (FR)'
    },
    main_fragment: {
        dashboard: 'Tableau de bord',
        basic: 'Information de base',
        settings: 'Paramètres'
    },
    dashboard: {
        root_impl: 'Implementation root',
        zygote_monitor: 'Moniteur Zygote',
        modules: 'Aucun module | Module (1) | Modules ({0})',
        root_impl_normal: 'L'implémentation root actuelle est {impl}, denylist fonctionnera correctement.',
        root_impl_abnormal: 'Impossible de déterminer l' implantation root, denylist ne fonctionnera pas.',
        root_impl_multiple: 'Multiples implantations root trouvées , denylist ne fonctionnera pas.',
        kernelsu_denylist: 'Denylist de KernelSU fait référence aux apps marquées \'Unmount modules\' dans le Profil des apps.',
        magisk_denylist: 'Denylist de Magisk fait référence à la liste d'exclusions interne de Magisk.',
        apatch_denylist: 'Denylist de APatch fait référence aux apps SuperUtilisateur pour lesquelles \'Exclude\' est autorisé et inhiber root',
    },
    settings: {
        log_to_kernel: 'Traces via dmesg (seulement pour les développeurs)',
        nonroot_as_denylist: 'Treat non-root apps as denylist',
        enforce_denylist: 'Denylist Policy',
        enforce_denylist_desc: 'Enforced: any modification for apps in denylist will be reverted.<br/>Umount Only: only mount modifications for apps in denylist will be reverted.',
        denylist_disabled: 'Disabled',
        denylist_enforced: 'Enforced',
        denylist_just_umount: 'Unmount Only',
        anonymous_memory: 'Use anonymous memory',
        anonymous_memory_desc: 'Load modules into anonymous memory. This compromises log readability but circumvents certain outdated detection mechanisms.',
        zn_linker: 'Use Zygisk Next linker (Experimental)',
        zn_linker_desc: 'Use built-in linker instead of the system linker to load modules. This will enhance stealth but may cause compatibility issues.',
    },
    zygote_inject_state: {
        running: 'Running',
        stop_by_user: 'Stop by user',
        stop_by_crash: 'Stop by zygote crashed',
        running_desc: 'Zygote Monitor is running normally.',
        stop_by_user_desc: 'Zygote Monitor stopped by user.',
        stop_by_crash_desc: 'Repeated restarts of Zygote has been detected, Zygote Monitor has automatically stopped.',
    },
    zygote_state: {
        unknown: 'Inconnu',
        injected: 'Injecté ({pid})',
        inject_failed: 'Échec injection ({pid})',
        skipped: 'Ignoré ({pid})',
        unknown_desc: 'The status of this Zygote is unknown and may exist but not be started, or the Zygote monitor may not detect its start.',
        injected_desc: 'Zygisk has been injected into the Zygote. Its process ID is {pid}',
        inject_failed_desc: 'Zygisk was attempted to be injected into this Zygote but failed. Its process ID is {pid}',
        skipped_desc: 'The Zygote was monitored to start. Its process ID is {pid}. But Zygisk injecting was stopped due to multiple previous soft reboots of the system.'
    },
    corrupted: {
        title: 'Fichiers de module corrompus',
        desc: 'Veuillez annuler les changements de Zygisk Next et réessayez.'
    }
}
