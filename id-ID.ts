export default {
    language: { 
        name: 'Indonesia (ID)'
    },
    main_fragment: {
        dashboard: 'Dasbor',
        basic: 'Informasi Dasar',
        settings: 'Pengaturan'
    },
    dashboard: {
        root_impl: 'Implementasi Root',
        zygote_monitor: 'Monitor Zygote',
        modules: 'Tidak ada Modul | Modul (1) | Modul ({0})',
        root_impl_normal: 'Root saat ini {impl}, denylist akan berfungsi dengan normal.',
        root_impl_abnormal: 'Tidak dapat menentukan Implementasi Root, sehingga denylist tidak akan berfungsi.',
        root_impl_multiple: 'Ditemukan lebih dari satu Implementasi Root, denylist tidak akan berfungsi.',
        kernelsu_denylist: 'Denylist di KernelSU mengacu pada aplikasi yang ditandai sebagai \'Unmount modul\' di profil aplikasi.',
        magisk_denylist: 'Denylist di Magisk mengacu pada denylist bawaan Magisk\’s.',
        apatch_denylist: 'Denylist di APatch mengacu pada aplikasi SuperUser yang \'Kecualikan\' diaktifkan dan menonaktifkan root.',
    },
    settings: {
        log_to_kernel: 'Log ke dmesg (Hanya untuk Pengembang)',
        nonroot_as_denylist: 'Anggap aplikasi non-root sebagai denylist',
        enforce_denylist: 'Kebijakan Denylist',
        enforce_denylist_desc: 'Dipaksakan: semua perubahan pada app di denylist bakal dibalikin.<br/>Hanya Umount: hanya perubahan mount di app denylist yang bakal dibalikin.',
        denylist_disabled: 'Nonaktif',
        denylist_enforced: 'Dipaksakan',
        denylist_just_umount: 'Hanya Unmount',
        anonymous_memory: 'Gunakan anonim memori',
        anonymous_memory_desc: 'Muat modul ke memori anonim. Ini bikin log jadi susah dibaca tapi bisa ngelewatin beberapa sistem deteksi lama.',
        zn_linker: 'Gunakan linker Zygisk Next (Eksperimental)',
        zn_linker_desc: 'Gunakan linker bawaan daripada linker sistem untuk memuat modul. Meningkatkan penyamaran, tapi bisa menyebabkan masalah kompatibilitas.',
    },
    zygote_inject_state: {
        running: 'Berjalan',
        stop_by_user: 'Dihentikan oleh pengguna',
        stop_by_crash: 'Dihentikan karena zygote crash',
        running_desc: 'Monitor Zygote Berjalan Normal.',
        stop_by_user_desc: 'Monitor Zygote dihentikan oleh pengguna.',
        stop_by_crash_desc: 'Terdeteksi Zygote terus-menerus memulai ulang, Monitor Zygote otomatis dihentikan.',
    },
    zygote_state: {
        unknown: 'Tidak diketahui',
        injected: 'Diinjeksi ({pid})',
        inject_failed: 'Gagal diinjeksi ({pid})',
        skipped: 'Dilewati ({pid})',
        unknown_desc: 'Status Zygote ini tidak diketahui, Mungkin ada tapi belum dijalankan, Atau Monitor Zygote tidak bisa mendeteksi saat dijalankan.',
        injected_desc: 'Zygisk sudah diinjeksi ke Zygote. ID prosesnya {pid}',
        inject_failed_desc: 'Zygisk dicoba diinjeksi ke Zygote ini tapi gagal. ID prosesnya {pid}',
        skipped_desc: 'Zygote dipantau untuk dijalankan. ID prosesnya {pid}. Tapi injeksi Zygisk dihentikan karena sistem sebelumnya beberapa kali soft reboot.'
    },
    corrupted: {
        title: 'File modul korup',
        desc: 'Silakan kembalikan perubahan di Zygisk Next dan coba lagi.'
    }
}
