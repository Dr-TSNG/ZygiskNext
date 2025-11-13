export default {
    // ZygiskNext/fr-FR.ts - French translations for ZygiskNext
    // (c) 2025 Gozer-404 https://github.com/Gozer-404
    language: { 
        name: 'Français (FR)'
    },
    main_fragment: {
        dashboard: 'Tableau de bord',
        basic: 'Informations de base',
        settings: 'Paramètres'
    },
    dashboard: {
        root_impl: 'Implementation root',
        zygote_monitor: 'Moniteur Zygote',
        modules: 'Aucun module | Module (1) | Modules ({0})',
        root_impl_normal: 'L\'implémentation root actuelle est {impl}, la liste d\'exclusions fonctionnera correctement.',
        root_impl_abnormal: 'Impossible de déterminer l\'implémentation root, la liste d\'exclusions ne fonctionnera pas.',
        root_impl_multiple: 'Multiples implantations root trouvées, la liste d\'exclusions ne fonctionnera pas.',
        kernelsu_denylist: 'La liste d\'exclusions de KernelSU fait référence aux apps marquées « Unmount modules » dans le profil des apps.',
        magisk_denylist: 'La liste d\'exclusions de Magisk fait référence à la liste d\'exclusions interne de Magisk.',
        apatch_denylist: 'La liste d\'exclusions de APatch fait référence aux apps SuperUtilisateur pour lesquelles « Exclude » est activé et inhiber root',
    },
    settings: {
        log_to_kernel: 'Traces via dmesg (seulement pour les développeurs)',
        nonroot_as_denylist: 'Traiter les apps non-root comme dans liste d\'exclusions',
        enforce_denylist: 'Stratégie liste d\'exclusions',
        enforce_denylist_desc: 'Forcée: toute modification d\'une app dans la liste d\'exclusions sera rétablie.<br/>Seulement « unmount »: seules les modifications pour « mount » des apps dans la liste d\'exclusions seront rétablies.',
        denylist_disabled: 'Inhibée',
        denylist_enforced: 'Forcée',
        denylist_just_umount: 'Seulement « unmount »',
        anonymous_memory: 'Utiliser une mémoire anonyme',
        anonymous_memory_desc: 'Charge les modules dans une mémoire anonyme. Cela compromet la lecture des traces mais contourne certains anciens mécanismes de détection.',
        zn_linker: 'Utiliser le linker Zygisk Next (expérimental)',
        zn_linker_desc: 'Utiliser le linker interne plutôt que celui du système pour charger des modules. Cela améliore la furtivité mais peut causer des problèmes d\'incompatibilité.',
    },
    zygote_inject_state: {
        running: 'Actif',
        stop_by_user: 'Stoppé par l\'utilsateur',
        stop_by_crash: 'Stoppé par un crash de Zygote',
        running_desc: 'Le moniteur Zygote fonctionne normalement.',
        stop_by_user_desc: 'Le moniteur Zygote est stoppé par l\'utilisateur.',
        stop_by_crash_desc: 'Redémmarages multiples de Zygote détectés, le moniteur Zygote a stoppé automatiquement.',
    },
    zygote_state: {
        unknown: 'Inconnu',
        injected: 'Injecté ({pid})',
        inject_failed: 'Échec injection ({pid})',
        skipped: 'Ignoré ({pid})',
        unknown_desc: 'L\'état de ce Zygote est inconnu (existe potentiellement mais ne peut être démarré), ou le moniteur Zygote n\'a pas détecté son démarrage.',
        injected_desc: 'Zygisk est injecté dans le Zygote. Son ID de process est {pid}',
        inject_failed_desc: 'Échec de l\'injection de Zygisk dans le Zygote. Son ID de process est {pid}',
        skipped_desc: 'Le Zygote a été detecté comme démarré. Son ID de process est {pid}. Mais l\'injection  Zygisk injecting a été stoppée par de multiple reboots du système.'
    },
    corrupted: {
        title: 'Fichiers de module corrompus',
        desc: 'Veuillez annuler les changements de Zygisk Next et réessayez.'
    }
}
