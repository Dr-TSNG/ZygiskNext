export default {
    // ZygiskNext/fr-FR.ts - French translations for ZygiskNext
    // (c) 2025 Gozer-404 https://github.com/Gozer-404
    // (c) 2026 PifGadget92 https://github.com/PifGadget92
    language: { 
        name: 'Français (FR)'
    },
    main_fragment: {
        dashboard: 'État',
        basic: 'Informations de base',
        settings: 'Paramètres'
    },
    bugreport: {
        export: 'Exporter le rapport de bugs',
        exporting: 'Exportation…',
        export_success_title: 'Rapport exporté',
        export_success: 'Raport enregistré dans : ',
        export_failed_title: 'Échec de l\'exportation',
        export_failed: 'Exportation du rapport de bugs impossible.',
        copy_path: 'Copier le chemin',
        copy_path_failed: 'Échec de la copie',
        send_log: 'Envoyer le journal',
        send_failed: 'Échec de l\'envoi',
    },
    dashboard: {
        root_impl: 'Implémentation root',
        zygote_monitor: 'Moniteur Zygote',
        zygisk_module_title: 'Aucun module Zygisk | Module Zygisk ({0}) | Modules Zygisk ({0})',
        zn_module_title: 'Aucun module ZN | Module ZN ({0}) | Modules ZN ({0})',
        root_impl_normal: 'L\'implémentation root utilisée est {impl}, la liste d\'exclusions fonctionnera correctement.',
        root_impl_abnormal: 'Impossible de déterminer l\'implémentation root, la liste d\'exclusions ne fonctionnera pas.',
        root_impl_multiple: 'Plusieurs implémentations root sont installées, la liste d\'exclusions ne fonctionnera pas.',
        kernelsu_denylist: 'La liste d\'exclusions de KernelSU fait référence aux applications dont l\'option \"Démontage des modules\" ou \"Unmount modules\" est activée dans leur profil.',
        magisk_denylist: 'La liste d\'exclusions de Magisk fait référence à la liste d\'exclusions interne de Magisk.',
        apatch_denylist: 'La liste d\'exclusions de APatch fait référence aux applications Superutilisateur pour lesquelles l\'option \"Exclude\" est activée et le root désactivé.',
    },
    settings: {
        log_to_kernel: 'Traces via dmesg (seulement pour les développeurs)',
        nonroot_as_denylist: 'Traiter les applications non-root comme étant dans liste d\'exclusions',
        enforce_denylist: 'Stratégie de liste d\'exclusions',
        enforce_denylist_desc: 'Appliquée : Bloque l\'injection de code et annule les modifications de montage pour les applications figurant sur la liste d\'exclusions.<br/>\"Umount\" uniquement : Annule les modifications de montage mais permet l\'injection de code pour les applications figurant sur la liste d\'exclusions.',
        enforce_denylist_alert: 'Pour les utilisateurs normaux, il est fortement recommandé de désactiver manuellement la fonctionnalité de démontage (\"umount\") du noyau dans le gestionnaire KernelSU pour éviter que les points de montage ne soient démontés plusieurs fois dans le cas de mauvaises configurations.',
        denylist_disabled: 'Désactivée',
        denylist_enforced: 'Appliquée',
        denylist_just_umount: '\"Umount\" uniquement',
        anonymous_memory: 'Utiliser une mémoire anonyme',
        anonymous_memory_desc: 'Charge les modules dans une mémoire anonyme. Cela compromet la lecture des traces mais contourne certains anciens mécanismes de détection.',
        zn_linker: 'Utiliser le linker de Zygisk Next',
        zn_linker_desc: 'Utilise le linker interne plutôt que celui du système pour charger les modules, ce qui permet d\'améliorer la furtivité mais peut provoquer des problèmes d\'incompatibilité.',
    },
    zygote_inject_state: {
        running: 'Actif',
        stop_by_user: 'Arrêté par l\'utilsateur',
        stop_by_crash: 'Arrêté par un plantage de Zygote',
        running_desc: 'Le moniteur Zygote fonctionne normalement.',
        stop_by_user_desc: 'Le moniteur Zygote a été arrêté par l\'utilisateur.',
        stop_by_crash_desc: 'Redémarrages multiples de Zygote détectés, le moniteur Zygote s\'est arrêté automatiquement.',
    },
    zygote_state: {
        unknown: 'Inconnu',
        injected: 'Injecté ({pid})',
        inject_failed: 'Échec injection ({pid})',
        skipped: 'Ignoré ({pid})',
        abnormal: 'Anormal ({pid})',
        unknown_desc: 'L\'état de ce Zygote est inconnu (il existe peut-être mais ne peut être démarré), ou le moniteur Zygote n\'a pas détecté son démarrage.',
        injected_desc: 'Zygisk est injecté dans le Zygote. Son ID de processus est {pid}',
        inject_failed_desc: 'Échec de l\'injection de Zygisk dans le Zygote. Son ID de processus est {pid}',
        skipped_desc: 'Le Zygote a été detecté comme démarré. Son ID de processus est {pid}. Mais l\'injection de Zygisk a été interrompue par de multiples redémarrages logiciels du système.',
        abnormal_desc: 'Zygisk a été injecté dans le processus Zygote {pid}, mais l\'interception (hook) par le JNI n\'a trouvé aucune méthode correspondante ({funcs}), les modules ne pourront donc pas se charger normalement. Il peut s\'agir d\'une nouvelle version non prise en charge ou d\'un système Android particulier. Créer une issue sur GitHub et fournir les fichiers /system/framework/framework.jar et /system/lib64/libandroid_runtime.so du système afin que les développeurs puissent en ajouter la prise en charge.'
    },
    corrupted: {
        title: 'Fichiers du module corrompus',
        desc: 'Annuler les modifications de Zygisk Next et essayer à nouveau.'
    },
    module: {
        issue: {
            title: 'Ce module présente un problème',
            companion_api_issue: 'Le module {name} présente un problème d\'utilisation incorrecte de l\'API Companion, ce qui peut provoquer des plantages de processus et des fuites de mémoire. Contacter le développeur de ce module pour résoudre le problème.',
            linker_issue: 'Le module {name} n\'a pas pu être chargé. Signaler le problème au développeur du module.',
            crash_issue: 'Le module {name} apparaît dans la trace d\'appel du thread en échec de : {processes}.',
            unknown_process: 'processus inconnu',
            learn_more: 'Pour plus d\'informations, consulter ：{link}',
            check_banner: 'Aucun module problématique détecté. | {0} module présente un problème. Vérifier la liste des modules. | {0} modules présentent des problèmes. Vérifier la liste des modules.',
            badge: 'Problème',
            crash_badge: 'Plantage',
        },
        zn: {
            process_count: 'Aucun processus | {0} processus | {0} processus',
        },
    }
}
