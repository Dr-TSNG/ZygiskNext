export default {
    language: { 
        name: 'Türkçe (TR)'
    },
    main_fragment: {
        dashboard: 'Durum',
        basic: 'Temel Bilgiler',
        settings: 'Ayarlar'
    },
    dashboard: {
        root_impl: 'Root uygulaması',
        zygote_monitor: 'Zygote Monitörü',
        zygisk_module_title: 'Zygisk modülü yok | Zygisk modülü ({0}) | Zygisk modülleri ({0})',
        zn_module_title: 'ZN modülü yok | ZN modülü ({0}) | ZN modülleri ({0})',
        root_impl_normal: 'Mevcut Root Uygulaması {impl}, engelleme listesi düzgün çalışacaktır.',
        root_impl_abnormal: 'Root Uygulaması belirlenemedi, engelleme listesi çalışmayacak.',
        root_impl_multiple: 'Birden fazla Root uygulaması bulundu, engelleme listesi çalışmayacaktır.',
        kernelsu_denylist: 'KernelSU`nun reddetme listesi, Uygulama Profilindeki \‘Modülleri kaldır\’ olarak işaretlenmiş uygulamaları ifade eder.',
        magisk_denylist: 'Magisk`in engelleme listesi, Magisk`in yerleşik engelleme listesini ifade eder.',
        apatch_denylist: 'APatch`in engelleme listesi, \‘Hariç Tut\’ seçeneğinin etkinleştirildiği ve Rootun devre dışı bırakıldığı SuperUser uygulamasını ifade eder.',
    },
    settings: {
        log_to_kernel: 'Dmesg`e log kaydet (Yalnızca geliştiriciler için)',
        nonroot_as_denylist: 'Root erişimi olmayan uygulamaları engellenenler listesi olarak değerlendir',
        enforce_denylist: 'Engelleme Listesi Politikası',
        enforce_denylist_desc: 'Zorlanmış: reddedilen listesindeki uygulamalar için yapılan tüm değişiklikler geri alınacaktır. <br/>Yalnızca umount: reddedilen listesindeki uygulamalar için sadece montaj değişiklikleri geri alınacaktır.',
        denylist_disabled: 'Devre Dışı',
        denylist_enforced: 'Zorlanmış',
        denylist_just_umount: 'Yalnızca Unmount',
        anonymous_memory: 'Anonim bellek kullan',
        anonymous_memory_desc: 'Modülleri anonim belleğe yükleyin. Bu, log okunabilirliğini tehlikeye atar, ancak bazı eski algılama mekanizmalarını atlatır.',
        zn_linker: 'Zygisk Next bağlayıcıyı kullanın',
        zn_linker_desc: 'Modülleri yüklemek için sistem bağlayıcı yerine yerleşik bağlayıcıyı kullanın. Bu, gizliliği artıracak ancak uyumluluk sorunlarına neden olabilir.',
    },
    zygote_inject_state: {
        running: 'Çalışıyor',
        stop_by_user: 'Kullanıcı tarafından durduruldu',
        stop_by_crash: 'Zygote çöktü',
        running_desc: 'Zygote Monitörü normal şekilde çalışıyor.',
        stop_by_user_desc: 'Zygote Monitörü kullanıcı tarafından durduruldu.',
        stop_by_crash_desc: 'Zygote`un tekrar tekrar yeniden başlatıldığı tespit edildi, Zygote Monitörü otomatik olarak durduruldu.',
    },
    zygote_state: {
        unknown: 'Bilinmiyor',
        injected: 'Enjekte edildi ({pid})',
        inject_failed: 'Enjeksiyon başarısız ({pid})',
        skipped: 'Atlandı ({pid})',
        unknown_desc: 'Bu Zygote`un durumu bilinmiyor ve mevcut olabilir ancak başlatılmamış olabilir veya Zygote monitörü onun başlatıldığını algılamamış olabilir.',
        injected_desc: 'Zygisk, Zygote`a enjekte edildi. İşlem kimliği {pid}',
        inject_failed_desc: 'Zygisk bu Zygote`a enjekte edilmeye çalışıldı ancak başarısız oldu. İşlem kimliği {pid}',
        skipped_desc: 'Zygote`un başlatılması izlendi. İşlem kimliği {pid}. Ancak, sistemin önceki birçok yumuşak yeniden başlaması nedeniyle Zygisk enjeksiyonu durduruldu.'
    },
    corrupted: {
        title: 'Modül dosyaları bozulmuş',
        desc: 'Lütfen Zygisk Next`teki değişiklikleri geri alın ve tekrar deneyin.'
    },
    module: {
        issue: {
            title: 'Bu modülün bir sorunu var',
            companion_api_issue: 'Bu modül {name}, Companion API\'nin yanlış kullanılması ile ilgili bir soruna sahiptir ve işlem çökmelerine ve bellek sızıntılarına neden olabilir. Lütfen bu modülün geliştiricisine başvurun.',
            learn_more: 'Daha fazla bilgi için buraya ziyaret edin：{link}',
            check_banner: 'Sorunlu modül tespit edilmedi. | {0} sorunlu modül tespit edildi. Lütfen modül listesini kontrol edin. | {0} sorunlu modül tespit edildi. Lütfen modül listesini kontrol edin.',
            badge: 'Sorunlu',
        },
        zn: {
            process_count: 'Süreç yok | {0} süreç | {0} süreç',
        },
    }
}
