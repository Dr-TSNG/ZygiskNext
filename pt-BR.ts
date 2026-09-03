export default {
    language: {
        name: 'Português (Brasil)'
    },
    main_fragment: {
        dashboard: 'Status',
        basic: 'Informações Básicas',
        settings: 'Configurações'
    },
    bugreport: {
        export: 'Exportar relatório de bugs',
        exporting: 'Exportando…',
        export_success_title: 'Relatório de bugs exportado',
        export_success: 'O relatório de bugs foi salvo em:',
        export_failed_title: 'Falha na exportação',
        export_failed: 'Falha ao exportar o relatório de bugs.',
        copy_path: 'Copiar caminho',
        copy_path_failed: 'Falha ao copiar',
        send_log: 'Enviar log',
        send_failed: 'Falha ao enviar',
    },
    dashboard: {
        root_impl: 'Implementação de root',
        zygote_monitor: 'Monitor do Zygote',
        zygisk_module_title: 'Nenhum módulo Zygisk | {0} módulo Zygisk | {0} módulos Zygisk',
        zn_module_title: 'Nenhum módulo ZN | {0} módulo ZN | {0} módulos ZN',
        root_impl_normal: 'A implementação de root atual é {impl}, a denylist vai funcionar corretamente.',
        root_impl_abnormal: 'Não foi possível determinar a implementação de root, a denylist não vai funcionar.',
        root_impl_multiple: 'Múltiplas implementações de root encontradas, a denylist não vai funcionar.',
        kernelsu_denylist: 'A denylist do KernelSU se refere a apps marcados como \\'Desmontar módulos\\' no App Profile.',
        magisk_denylist: 'A denylist do Magisk se refere à denylist embutida do Magisk.',
        apatch_denylist: 'A denylist do APatch se refere ao SuperUser App com \\'Excluir\\' ativado e root desativado',
    },
    settings: {
        log_to_kernel: 'Registrar no dmesg (Apenas para desenvolvedores)',
        nonroot_as_denylist: 'Tratar apps sem root como denylist',
        enforce_denylist: 'Política da Denylist',
        enforce_denylist_desc: 'Forçada: bloqueia injeção de código e reverte modificações de mount para apps na denylist.<br/>Apenas Desmontar: reverte modificações de mount mas permite injeção de código para apps na denylist.',
        enforce_denylist_alert: 'Para usuários comuns, é altamente recomendado desativar manualmente o recurso de umount do kernel no gerenciador do KernelSU, para evitar que pontos de montagem sejam desmontados várias vezes por configuração incorreta.',
        denylist_disabled: 'Desativada',
        denylist_enforced: 'Forçada',
        denylist_just_umount: 'Apenas Desmontar',
        anonymous_memory: 'Usar memória anônima',
        anonymous_memory_desc: 'Carrega módulos em memória anônima. Isso compromete a legibilidade dos logs mas contorna certos mecanismos de detecção desatualizados.',
        zn_linker: 'Usar o linker do Zygisk Next',
        zn_linker_desc: 'Usa o linker embutido em vez do linker do sistema para carregar módulos. Isso melhora o disfarce mas pode causar problemas de compatibilidade.',
    },
    zygote_inject_state: {
        running: 'Em execução',
        stop_by_user: 'Parado pelo usuário',
        stop_by_crash: 'Parado por crash do zygote',
        running_desc: 'O Monitor do Zygote está funcionando normalmente.',
        stop_by_user_desc: 'O Monitor do Zygote foi parado pelo usuário.',
        stop_by_crash_desc: 'Foram detectadas reinicializações repetidas do Zygote, o Monitor do Zygote parou automaticamente.',
    },
    zygote_state: {
        unknown: 'Desconhecido',
        injected: 'Injetado ({pid})',
        inject_failed: 'Falha na injeção ({pid})',
        skipped: 'Ignorado ({pid})',
        abnormal: 'Anormal ({pid})',
        unknown_desc: 'O status deste Zygote é desconhecido e pode existir mas não ter sido iniciado, ou o Monitor do Zygote pode não ter detectado seu início.',
        injected_desc: 'O Zygisk foi injetado no Zygote. Seu ID de processo é {pid}',
        inject_failed_desc: 'Houve uma tentativa de injetar o Zygisk neste Zygote, mas falhou. Seu ID de processo é {pid}',
        skipped_desc: 'O Zygote foi monitorado ao iniciar. Seu ID de processo é {pid}. Mas a injeção do Zygisk foi interrompida devido a múltiplas reinicializações leves anteriores do sistema.',
        abnormal_desc: 'O Zygisk foi injetado no processo Zygote {pid}, mas o JNI Hook não encontrou métodos correspondentes ({funcs}), então os módulos não vão carregar normalmente. Isso pode ser uma versão nova não suportada ou um sistema Android especial. Crie uma issue no GitHub e forneça os arquivos /system/framework/framework.jar e /system/lib64/libandroid_runtime.so do sistema atual para que os desenvolvedores possam adicionar suporte.'
    },
    corrupted: {
        title: 'Arquivos do módulo corrompidos',
        desc: 'Reverta as alterações no Zygisk Next e tente novamente.'
    },
    module: {
        issue: {
            title: 'Este módulo tem um problema',
            companion_api_issue: 'Este módulo {name} tem um problema de uso incorreto da Companion API, o que pode causar travamentos de processo e vazamentos de memória. Entre em contato com o desenvolvedor deste módulo para resolver o problema.',
            linker_issue: 'Este módulo {name} falhou ao carregar. Reporte o problema ao desenvolvedor do módulo.',
            crash_issue: 'Este módulo {name} apareceu no backtrace de travamento de: {processes}.',
            unknown_process: 'processo desconhecido',
            learn_more: 'Visite aqui para mais informações: {link}',
            check_banner: 'Nenhum módulo problemático detectado. | {0} módulo tem um problema. Revise a lista de módulos. | {0} módulos têm problemas. Revise a lista de módulos.',
            badge: 'Problema',
            crash_badge: 'Travamento',
        },
        zn: {
            process_count: 'Nenhum processo | {0} processo | {0} processos',
        },
    }
}
