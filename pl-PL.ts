export default {
    language: {
        name: 'Polski (PL)'
    },
    main_fragment: {
        dashboard: 'Status',
        basic: 'Podstawowe informacje',
        settings: 'Ustawienia'
    },
    bugreport: {
        export: 'Eksportuj raport błędu',
        exporting: 'Eksportowanie…',
        export_success_title: 'Raport błędu wyeksportowany',
        export_success: 'Raport błędu został zapisany w:',
        export_failed_title: 'Eksport nie powiódł się',
        export_failed: 'Nie udało się wyeksportować raportu błędu.',
        copy_path: 'Kopiuj ścieżkę',
        copy_path_failed: 'Kopiowanie nie powiodło się',
        send_log: 'Wyślij dziennik',
        send_failed: 'Wysyłanie nie powiodło się',
    },
    dashboard: {
        root_impl: 'Implementacja roota',
        zygote_monitor: 'Monitor Zygote',
        zygisk_module_title: 'Brak modułów Zygisk | Moduł Zygisk ({0}) | Moduły Zygisk ({0})',
        zn_module_title: 'Brak modułów ZN | Moduł ZN ({0}) | Moduły ZN ({0})',
        root_impl_normal: 'Bieżąca implementacja roota to {impl}; denylista będzie działać prawidłowo.',
        root_impl_abnormal: 'Nie można określić implementacji roota; denylista nie będzie działać.',
        root_impl_multiple: 'Znaleziono wiele implementacji roota; denylista nie będzie działać.',
        kernelsu_denylist: 'Denylista KernelSU odnosi się do aplikacji oznaczonych jako „Unmount modules” w profilu aplikacji.',
        magisk_denylist: 'Denylista Magisk odnosi się do wbudowanej denylisty Magisk.',
        apatch_denylist: 'Denylista APatch odnosi się do aplikacji SuperUser, dla których włączono opcję „Exclude” i wyłączono roota.',
    },
    settings: {
        log_to_kernel: 'Loguj do dmesg (tylko dla deweloperów)',
        nonroot_as_denylist: 'Traktuj aplikacje bez roota jak znajdujące się na denyliście',
        enforce_denylist: 'Zasady denylisty',
        enforce_denylist_desc: 'Wymuszona: blokuje wstrzykiwanie kodu i cofa modyfikacje montowania dla aplikacji na denyliście.<br/>Tylko odmontowanie: cofa modyfikacje montowania, ale pozwala na wstrzykiwanie kodu dla aplikacji na denyliście.',
        enforce_denylist_alert: 'Zwykłym użytkownikom zdecydowanie zaleca się ręczne wyłączenie funkcji odmontowywania jądra w menedżerze KernelSU, aby uniknąć wielokrotnego odmontowywania punktów montowania z powodu błędnej konfiguracji.',
        denylist_disabled: 'Wyłączona',
        denylist_enforced: 'Wymuszona',
        denylist_just_umount: 'Tylko odmontowanie',
        anonymous_memory: 'Użyj pamięci anonimowej',
        anonymous_memory_desc: 'Ładuje moduły do pamięci anonimowej. Pogarsza to czytelność dzienników, ale omija niektóre przestarzałe mechanizmy wykrywania.',
        zn_linker: 'Użyj linkera Zygisk Next',
        zn_linker_desc: 'Używa wbudowanego linkera zamiast linkera systemowego do ładowania modułów. Zwiększa to dyskrecję, ale może powodować problemy ze zgodnością.',
    },
    zygote_inject_state: {
        running: 'Działa',
        stop_by_user: 'Zatrzymano przez użytkownika',
        stop_by_crash: 'Zatrzymano z powodu awarii Zygote',
        running_desc: 'Monitor Zygote działa normalnie.',
        stop_by_user_desc: 'Monitor Zygote został zatrzymany przez użytkownika.',
        stop_by_crash_desc: 'Wykryto powtarzające się restarty Zygote; Monitor Zygote został automatycznie zatrzymany.',
    },
    zygote_state: {
        unknown: 'Nieznany',
        injected: 'Wstrzyknięto ({pid})',
        inject_failed: 'Wstrzyknięcie nie powiodło się ({pid})',
        skipped: 'Pominięto ({pid})',
        unknown_desc: 'Status tego procesu Zygote jest nieznany; może istnieć, ale nie być uruchomiony, albo monitor Zygote mógł nie wykryć jego startu.',
        injected_desc: 'Zygisk został wstrzyknięty do Zygote. Identyfikator procesu to {pid}',
        inject_failed_desc: 'Podjęto próbę wstrzyknięcia Zygisk do tego procesu Zygote, ale nie powiodła się. Identyfikator procesu to {pid}',
        skipped_desc: 'Monitor wykrył uruchomienie Zygote. Identyfikator procesu to {pid}. Wstrzykiwanie Zygisk zostało jednak zatrzymane z powodu wielu wcześniejszych miękkich restartów systemu.'
    },
    corrupted: {
        title: 'Pliki modułu są uszkodzone',
        desc: 'Cofnij zmiany w Zygisk Next i spróbuj ponownie.'
    },
    module: {
        issue: {
            title: 'Ten moduł ma problem',
            companion_api_issue: 'Moduł {name} ma problem z nieprawidłowym użyciem Companion API, co może powodować awarie procesów i wycieki pamięci. Skontaktuj się z deweloperem tego modułu, aby rozwiązać problem.',
            linker_issue: 'Nie udało się załadować modułu {name}. Zgłoś problem deweloperowi modułu.',
            crash_issue: 'Moduł {name} pojawił się w śladzie stosu wątku powodującego awarię dla: {processes}.',
            unknown_process: 'nieznany proces',
            learn_more: 'Więcej informacji znajdziesz tutaj: {link}',
            check_banner: 'Nie wykryto problematycznych modułów. | {0} moduł ma problem. Sprawdź listę modułów. | {0} moduły mają problemy. Sprawdź listę modułów.',
            badge: 'Problem',
            crash_badge: 'Awaria',
        },
        zn: {
            process_count: 'Brak procesów | {0} proces | {0} procesy',
        },
    }
}
