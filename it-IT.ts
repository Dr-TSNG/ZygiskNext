export default {
    language: {
        name: 'Italiano (IT)',
    },
    main_fragment: {
        dashboard: 'Stato',
        basic: 'Informazioni di base',
        settings: 'Impostazioni',
    },
    bugreport: {
        export: 'Esporta bugreport',
        exporting: 'Esportazione in corso…',
        export_success_title: 'Bugreport esportato',
        export_success: 'Il bugreport è stato salvato in:',
        export_failed_title: 'Esportazione non riuscita',
        export_failed: 'Impossibile esportare il bugreport.',
        copy_path: 'Copia percorso',
        copy_path_failed: 'Impossibile copiare il percorso',
        send_log: 'Invia log',
        send_failed: 'Invio non riuscito',
    },
    dashboard: {
        root_impl: 'Implementazione root',
        zygote_monitor: 'Monitor Zygote',
        zygisk_module_title: 'Nessun modulo Zygisk | Modulo Zygisk ({0}) | Moduli Zygisk ({0})',
        zn_module_title: 'Nessun modulo ZN | Modulo ZN ({0}) | Moduli ZN ({0})',
        root_impl_normal: "L'implementazione root attuale è {impl}, la lista d'esclusione funzionerà correttamente.",
        root_impl_abnormal:
            "Non è stato possibile determinare l'implementazione root, la lista d'esclusione non funzionerà.",
        root_impl_multiple: "Trovate più implementazioni root, la lista d'esclusione non funzionerà.",
        kernelsu_denylist:
            "La lista d'esclusione di KernelSU si riferisce alle app contrassegnate con «Unmount modules» nel profilo app.",
        magisk_denylist: "La lista d'esclusione di Magisk si riferisce alla lista d'esclusione all'interno di Magisk.",
        apatch_denylist:
            "La lista d'esclusione di APatch si riferisce alle app nella sezione Superuser con l'opzione «Exclude» attiva e senza accesso root.",
    },
    settings: {
        log_to_kernel: 'Registra i log in dmesg (solo per sviluppatori)',
        nonroot_as_denylist: "Includi le app senza accesso root nella lista d'esclusione",
        enforce_denylist: "Politica della lista d'esclusione",
        enforce_denylist_desc:
            "Forzata: Blocca l'iniezione di codice e annulla le modifiche ai montaggi per le app nella lista d'esclusione.<br/>Solo smontaggio: Annulla le modifiche ai montaggi ma consente l'iniezione di codice per le app nella lista d'esclusione.",
        enforce_denylist_alert:
            'Per gli utenti comuni è fortemente consigliato disabilitare manualmente la funzione di smontaggio del kernel nel gestore di KernelSU, per evitare che i punti di montaggio vengano smontati più volte a causa di una configurazione errata.',
        denylist_disabled: 'Disabilitata',
        denylist_enforced: 'Forzata',
        denylist_just_umount: 'Solo smontaggio',
        anonymous_memory: 'Occultamento della memoria',
        anonymous_memory_desc:
            'Carica i moduli nella memoria anonima o nasconde le loro mappature in procfs. Questo compromette la leggibilità dei log ma aggira alcuni meccanismi di rilevamento obsoleti.<br/>Attivazione: ha effetto dopo il riavvio del demone. Disattivazione: richiede il riavvio del dispositivo.',
        memory_type_default: 'Predefinita',
        memory_type_anonymous: 'Anonima',
        memory_type_hidden: 'Nascondi le mappature',
        memory_type_restart:
            "Per attivare l'occultamento delle mappature, riavvia il demone. Per disattivarlo, riavvia il dispositivo.",
        memory_type_save_failed: 'Impossibile salvare la modalità di memoria',
        maps_hiding_unknown: "Lo stato iniziale dell'occultamento delle mappature non è disponibile.",
        maps_hiding_disabled: "L'occultamento delle mappature non era attivo all'avvio del demone.",
        maps_hiding_ready: "L'occultamento delle mappature è stato inizializzato correttamente all'avvio del demone.",
        maps_hiding_failed:
            "Impossibile inizializzare l'occultamento delle mappature all'avvio del demone. Verifica che sia installato un modulo del kernel compatibile.",
        zn_linker: 'Usa il linker di Zygisk Next',
        zn_linker_desc:
            "Usa il linker integrato invece del linker di sistema per caricare i moduli. Questo migliorerà l'invisibilità ma potrebbe causare problemi di compatibilità.",
    },
    zygote_inject_state: {
        running: 'Attivo',
        stop_by_user: "Fermato dall'utente",
        stop_by_crash: 'Arrestato a causa di un crash di Zygote',
        running_desc: 'Il monitor di Zygote sta funzionando normalmente.',
        stop_by_user_desc: "Monitor di Zygote fermato dall'utente.",
        stop_by_crash_desc:
            'Sono stati rilevati ripetuti riavvii di Zygote. Il monitor di Zygote è stato fermato automaticamente.',
    },
    zygote_state: {
        unknown: 'Sconosciuto',
        injected: 'Iniettato ({pid})',
        inject_failed: 'Iniezione fallita ({pid})',
        skipped: 'Saltato ({pid})',
        abnormal: 'Anormale ({pid})',
        unknown_desc:
            'Lo stato di questo Zygote è sconosciuto. Potrebbe esistere ma non essere avviato, oppure il monitor di Zygote potrebbe non aver rilevato il suo avvio.',
        injected_desc: 'Zygisk è stato iniettato in Zygote. Il suo ID di processo è {pid}',
        inject_failed_desc:
            'Il tentativo di iniettare Zygisk in questo Zygote è fallito. Il suo ID di processo è {pid}',
        skipped_desc:
            "È stato rilevato l'avvio di Zygote. Il suo ID di processo è {pid}. L'iniezione di Zygisk è stata fermata a causa di ripetuti riavvii del sistema senza riavvio del kernel.",
        abnormal_desc:
            "Zygisk è stato iniettato nel processo Zygote {pid}, ma l'hook JNI non ha trovato metodi corrispondenti ({funcs}), quindi i moduli non si caricheranno normalmente. Potrebbe trattarsi di una nuova versione non supportata o di un sistema Android particolare. Apri un issue su GitHub e fornisci /system/framework/framework.jar e /system/lib64/libandroid_runtime.so del sistema attuale, così che gli sviluppatori possano aggiungere il supporto.",
    },
    corrupted: {
        title: 'File del modulo danneggiati',
        desc: 'Annulla le modifiche a Zygisk Next e riprova.',
    },
    module: {
        issue: {
            title: 'Questo modulo ha un problema',
            companion_api_issue:
                "Il modulo {name} usa in modo improprio l'API Companion, il che potrebbe causare crash dei processi e perdite di memoria. Contatta lo sviluppatore di questo modulo per risolvere il problema.",
            linker_issue: 'Impossibile caricare il modulo {name}. Segnala il problema allo sviluppatore del modulo.',
            crash_issue:
                'Il modulo {name} è apparso nel backtrace del thread in crash dei seguenti processi: {processes}.',
            unknown_process: 'processo sconosciuto',
            learn_more: 'Visita qui per ulteriori informazioni: {link}',
            check_banner:
                'Nessun modulo con problemi trovato. | {0} modulo ha un problema. Controlla la lista dei moduli. | {0} moduli hanno problemi. Controlla la lista dei moduli.',
            badge: 'Problema',
            crash_badge: 'Crash',
        },
        zn: {
            process_count: 'Nessun processo | {0} processo | {0} processi',
        },
    },
}
