export default {
    language: { 
        name: 'Bahasa Indonesia (ID)'
    },
    main_fragment: {
        dashboard: 'Dasbor',
        basic: 'Informasi Dasar',
        settings: 'Pengaturan'
    },
    dashboard: {
        root_impl: 'Implementasi root',
        zygote_monitor: 'Monitor Zygote',
        modules: 'Tidak ada modul | Modul (1) | Modul ({0})',
        root_impl_normal: 'Root saat ini {impl}, denylist akan berfungsi dengan normal.',
        root_impl_abnormal: 'Tidak dapat menentukan implementasi root, sehingga denylist tidak akan berfungsi.',
        root_impl_multiple: 'Ditemukan lebih dari satu implementasi root, denylist tidak akan berfungsi.',
        kernelsu_denylist: 'Denylist pada KernelSU mengacu pada aplikasi yang ditandai \'Umount modul\' pada profil aplikasi.',
        magisk_denylist: 'Denylist pada Magisk mengacu pada denylist bawaan Magisk.',
        apatch_denylist: 'Denylist pada APatch mengacu pada aplikasi tanpa akses root yang \'Kecualikan\'-nya aktif pada tab SuperUser.'
    },
    settings: {
        log_to_kernel: 'Log ke dmesg (Hanya untuk pengembang)',
        nonroot_as_denylist: 'Perlakukan aplikasi tanpa akses root sebagai denylist',
        enforce_denylist: 'Penerapan denylist',
        enforce_denylist_desc: 'Terapkan: segala perubahan yang diterapkan pada aplikasi dalam denylist akan dipulihkan<br/>Hanya umount: hanya perubahan terkait mount pada aplikasi dalam denylist yang akan dipulihkan.',
        denylist_disabled: 'Nonaktif',
        denylist_enforced: 'Terapkan',
        denylist_just_umount: 'Hanya unmount',
        anonymous_memory: 'Gunakan memori anonim',
        anonymous_memory_desc: 'Muat module ke dalam memori anonim. Akan berdampak terhadap keterbacaan log, namun berbagai mekanisme deteksi lama dapat diatasi.',
        zn_linker: 'Gunakan linker Zygisk Next (Eksperimental)',
        zn_linker_desc: 'Gunakan linker bawaan sebagai pengganti linker sistem untuk memuat modul. Penyamaran akan diperkuat tetapi dapat menyebabkan masalah kompatibilitas.'
    },
    zygote_inject_state: {
        running: 'Berjalan',
        stop_by_user: 'Dihentikan oleh pengguna',
        stop_by_crash: 'Terhenti karena Zygote crash',
        running_desc: 'Monitor Zygote berjalan normal.',
        stop_by_user_desc: 'Monitor Zygote dihentikan oleh pengguna.',
        stop_by_crash_desc: 'Zygote terdeteksi memulai ulang berulang kali, Monitor Zygote dinonaktifkan secara otomatis.'
    },
    zygote_state: {
        unknown: 'Tidak diketahui',
        injected: 'Diinjeksi ({pid})',
        inject_failed: 'Gagal diinjeksi ({pid})',
        skipped: 'Dilewatkan ({pid})',
        unknown_desc: 'Status Zygote ini tidak diketahui, Mungkin ada namun belum dimulai, atau monitor Zygote tidak mendeteksi permulaannya.',
        injected_desc: 'Zygote ini telah diinjeksikan Zygisk. ID prosesnya adalah {pid}',
        inject_failed_desc: 'Zygote ini telah dicoba untuk diinjeksikan Zygisk tetapi gagal. ID prosesnya adalah {pid}',
        skipped_desc: 'Zygote ini terpantau telah dimulai. ID prosesnya adalah {pid}. Namun injeksi Zygisk dihentikan karena sistem mengalami beberapa soft reboot sebelumnya.'
    },
    corrupted: {
        title: 'File modul rusak',
        desc: 'Harap urungkan perubahan pada file Zygisk Next dan coba lagi.'
    }
}
