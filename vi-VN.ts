export default {
    language: { 
        name: 'Tiếng Việt (VN)'
    },
    main_fragment: {
        dashboard: 'Trạng thái',
        basic: 'Thông tin Cơ bản',
        settings: 'Cài đặt'
    },
    dashboard: {
        root_impl: 'Trình thực thi Root',
        zygote_monitor: 'Trình giám sát Zygote',
        zygisk_module_title: 'Zygisk module ({0})',
        zn_module_title: 'ZN module ({0})',
        root_impl_normal: 'Trình thực thi Root hiện tại là {impl}, Denylist sẽ hoạt động bình thường.',
        root_impl_abnormal: 'Không thể xác định Trình thực thi Root, Denylist sẽ không hoạt động.',
        root_impl_multiple: 'Phát hiện nhiều Trình thực thi Root, Denylist sẽ không hoạt động.',
        kernelsu_denylist: 'Denylist của KernelSU đề cập đến các ứng dụng được đánh dấu là \'Unmount modules\' trong Hồ sơ ứng dụng (App Profile).',
        magisk_denylist: 'Denylist của Magisk đề cập đến Denylist được tích hợp sẵn của Magisk.',
        apatch_denylist: 'Denylist của APatch đề cập đến ứng dụng SuperUser khi tùy chọn \'Exclude (Loại trừ)\' được bật để vô hiệu hoá quyền root.'
    },
    settings: {
        log_to_kernel: 'Ghi nhật ký vào dmesg (Nhà phát triển)',
        nonroot_as_denylist: 'Xử lý các ứng dụng non-root như trong Denylist',
        enforce_denylist: 'Chính sách Denylist',
        enforce_denylist_desc: 'Enforced: Chặn tiêm mã và hoàn tác các thay đổi mount cho các ứng dụng trong Denylist.<br/>Unmount: Hoàn tác các thay đổi mount nhưng cho phép tiêm mã cho các ứng dụng trong Denylist.',
        enforce_denylist_alert: 'Đối với người dùng thông thường, chúng tôi đặc biệt khuyến nghị nên tắt tính năng Kernel Umount thủ công trong Trình quản lý KernelSU để tránh việc các điểm mount bị umount nhiều lần do cấu hình sai.',
        denylist_disabled: 'Tắt',
        denylist_enforced: 'Enforced',
        denylist_just_umount: 'Unmount',
        anonymous_memory: 'Sử dụng bộ nhớ ẩn danh',
        anonymous_memory_desc: 'Load các module vào bộ nhớ ẩn danh. Điều này làm giảm khả năng đọc nhật ký nhưng lại giúp tránh được một số cơ chế phát hiện lỗi thời.',
        zn_linker: 'Sử dụng Zygisk Next linker',
        zn_linker_desc: 'Dùng linker tích hợp sẵn thay cho linker của hệ thống để load các module. Cách này giúp tăng tính tàng hình nhưng có thể gây ra các vấn đề về khả năng tương thích.'
    },
    zygote_inject_state: {
        running: 'Đang chạy',
        stop_by_user: 'Dừng lại bởi người dùng',
        stop_by_crash: 'Dừng lại do Zygote bị crash',
        running_desc: 'Trình giám sát Zygote đang hoạt động bình thường.',
        stop_by_user_desc: 'Trình giám sát Zygote đã bị dừng lại bởi người dùng.',
        stop_by_crash_desc: 'Phát hiện Zygote khởi động lại nhiều lần, Trình giám sát Zygote đã tự động dừng lại.'
    },
    zygote_state: {
        unknown: 'Không xác định',
        injected: 'Đã tiêm ({pid})',
        inject_failed: 'Tiêm thất bại ({pid})',
        skipped: 'Đã bỏ qua ({pid})',
        unknown_desc: 'Trạng thái của Zygote này chưa được xác định và có thể tồn tại nhưng chưa được khởi chạy, hoặc Trình giám sát Zygote chưa phát hiện việc khởi động của nó.',
        injected_desc: 'Zygisk đã được tiêm vào Zygote. ID tiến trình là {pid}.',
        inject_failed_desc: 'Đã cố gắng tiêm Zygisk vào Zygote này nhưng thất bại. ID tiến trình là {pid}.',
        skipped_desc: 'Zygote đã được phát hiện khởi động. ID tiến trình là {pid}. Tuy nhiên việc tiêm Zygisk đã bị dừng lại do hệ thống gặp nhiều lần khởi động lại mềm trước đó.'
    },
    corrupted: {
        title: 'Tệp module bị hỏng',
        desc: 'Vui lòng hoàn tác các thay đổi đối với Zygisk Next và thử lại.'
    },
    module: {
        issue: {
            title: 'Module này có vấn đề',
            companion_api_issue: 'Module {name} này có vấn đề với việc sử dụng Companion API không đúng cách, có thể gây ra sự cố tiến trình và rò rỉ bộ nhớ. Vui lòng liên hệ nhà phát triển của module này để giải quyết vấn đề.',
            learn_more: 'Truy cập tại đây để biết thêm thông tin: {link}',
            check_banner: 'Đã phát hiện {0} module có vấn đề. Vui lòng kiểm tra danh sách module.',
            badge: 'Có vấn đề',
        },
        zn: {
            process_count: '{0} tiến trình',
        },
    }
}
