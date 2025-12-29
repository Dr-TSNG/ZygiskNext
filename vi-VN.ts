export default {
    language: { 
        name: 'Tiếng Việt'
    },
    main_fragment: {
        dashboard: 'Bảng điều khiển',
        basic: 'Thông tin cơ bản',
        settings: 'Cài đặt'
    },
    dashboard: {
        root_impl: 'Trình thực thi Root',
        zygote_monitor: 'Trình giám sát Zygote',
        modules: 'Không có module | Module (1) | Modules ({0})',
        root_impl_normal: 'Trình thực thi Root hiện tại là {impl}, Denylist sẽ hoạt động bình thường.',
        root_impl_abnormal: 'Không thể xác định trình thực thi Root, Denylist sẽ không hoạt động.',
        root_impl_multiple: 'Phát hiện nhiều trình thực thi Root, Denylist sẽ không hoạt động.',
        kernelsu_denylist: 'Denylist của KernelSU đề cập đến các ứng dụng được đánh dấu là \'Unmount modules\' trong Hồ sơ ứng dụng (App Profile).',
        magisk_denylist: 'Denylist của Magisk là Denylist tích hợp sẵn trong Magisk.',
        apatch_denylist: 'Denylist của APatch liên quan đến ứng dụng SuperUser khi tùy chọn \'Exclude\' được bật để vô hiệu hóa quyền root.'
    },
    settings: {
        log_to_kernel: 'Ghi log vào dmesg (Chỉ dành cho nhà phát triển)',
        nonroot_as_denylist: 'Xử lý ứng dụng non-root như Denylist',
        enforce_denylist: 'Chính sách cho Denylist',
        enforce_denylist_desc: 'Enforced: Chặn chèn mã và hoàn tác các thay đổi mount cho các ứng dụng trong denylist.<br/>Unmount: Hoàn tác các thay đổi mount nhưng cho phép chèn mã cho các ứng dụng trong denylist.',
        enforce_denylist_alert: 'Đối với người dùng thông thường, chúng tôi đặc biệt khuyến nghị nên tắt tính năng Umount Kernel thủ công trong Trình quản lý KernelSU để tránh việc các điểm umount nhiều lần do cấu hình sai.',
        denylist_disabled: 'Tắt',
        denylist_enforced: 'Enforced',
        denylist_just_umount: 'Unmount',
        anonymous_memory: 'Sử dụng bộ nhớ ẩn danh',
        anonymous_memory_desc: 'Load các module vào bộ nhớ ẩn danh. Điều này làm giảm khả năng đọc log nhưng giúp tránh một số cơ chế phát hiện cũ.',
        zn_linker: 'Sử dụng Zygisk Next Linker (Thử nghiệm)',
        zn_linker_desc: 'Dùng linker tích hợp sẵn thay cho linker của hệ thống để load các module. Cách này giúp tăng tính tàng hình nhưng có thể gây ra lỗi tương thích.'
    },
    zygote_inject_state: {
        running: 'Đang chạy',
        stop_by_user: 'Dừng bởi người dùng',
        stop_by_crash: 'Dừng do Zygote bị crash',
        running_desc: 'Trình giám sát Zygote đang hoạt động bình thường.',
        stop_by_user_desc: 'Trình giám sát Zygote đã bị dừng bởi người dùng.',
        stop_by_crash_desc: 'Phát hiện Zygote khởi động lại nhiều lần, trình giám sát Zygote đã tự động dừng.'
    },
    zygote_state: {
        unknown: 'Không xác định',
        injected: 'Đã tiêm ({pid})',
        inject_failed: 'Tiêm thất bại ({pid})',
        skipped: 'Bỏ qua ({pid})',
        unknown_desc: 'Trạng thái của Zygote này chưa được xác định và có thể tồn tại nhưng chưa được khởi chạy, hoặc trình giám sát Zygote chưa phát hiện việc khởi động của nó.',
        injected_desc: 'Zygisk đã được tiêm vào Zygote. ID tiến trình là {pid}.',
        inject_failed_desc: 'Đã cố gắng tiêm Zygisk vào Zygote này nhưng thất bại. ID tiến trình là {pid}.',
        skipped_desc: 'Zygote đã được phát hiện khởi động. ID tiến trình là {pid}. Tuy nhiên, việc tiêm Zygisk đã bị dừng do hệ thống gặp nhiều lần Soft Reboot trước đó.'
    },
    corrupted: {
        title: 'Tệp module bị hỏng',
        desc: 'Vui lòng hoàn tác các thay đổi đối với Zygisk Next và thử lại.'
    }
}
