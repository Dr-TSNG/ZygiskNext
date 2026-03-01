export default {
    language: { 
        name: 'Tiếng Việt (VN)'
    },
    main_fragment: {
        dashboard: 'Bảng điều khiển',
        basic: 'Thông tin Cơ bản',
        settings: 'Cài đặt'
    },
    dashboard: {
        root_impl: 'Triển khai Root',
        zygote_monitor: 'Trình giám sát Zygote',
        modules: 'Không có mô-đun nào | Mô-đun (1) | Mô-đun ({0})',
        root_impl_normal: 'Triẻn khai Root hiện tại là {impl}, Denylist sẽ hoạt động bình thường..',
        root_impl_abnormal: 'Không thể xác định Triển khai Root, Denylist sẽ không hoạt động.',
        root_impl_multiple: 'Nhiều Triển khai Root được tìm thấy, Denylist sẽ không hoạt động.',
        kernelsu_denylist: 'Denylist của KernelSU đề cập đến các ứng dụng được đánh dấu là \'Bỏ gắn kết mô-đun\' trong Hồ sơ Ứng dụng.',
        magisk_denylist: 'Denylist của Magisk đề cập đến Denylist được tích hợp sẵn của Magisk.',
        apatch_denylist: 'Denylist của APatch đề cập đến ứng dụng Superuser, trong đó tùy chọn \'Loại trừ\' được bật và root bị tắt.',
    },
    settings: {
        log_to_kernel: 'Ghi nhật ký vào dmesg (Chỉ dành cho nhà phát triển)',
        nonroot_as_denylist: 'Coi các ứng dụng không có root như trong Denylist.',
        enforce_denylist: 'Chính sách Denylist',
        enforce_denylist_desc: 'Thực thi: Chặn tiêm mã và hoàn tác các thay đổi gắn kết cho ứng dụng trong Denylist.<br/>Chỉ bỏ gắn kết: Hoàn tác các thay đổi gắn kết nhưng cho phép tiêm mã cho ứng dụng trong Denylist.',
        enforce_denylist_alert: 'Đối với người dùng thông thường, đặc biệt khuyến nghị nên tắt tính năng bỏ gắn kết kernel thủ công trong trình quản lý KernelSU để tránh việc các điểm gắn kết bị bỏ gắn kết nhiều lần do cấu hình sai.',
        denylist_disabled: 'Tắt',
        denylist_enforced: 'Thực thi',
        denylist_just_umount: 'Chỉ bỏ gắn kết',
        anonymous_memory: 'Sử dụng bộ nhớ ẩn danh',
        anonymous_memory_desc: 'Nạp mô-đun vào bộ nhớ ẩn danh. Điều này làm giảm khả năng đọc nhật ký nhưng lại giúp tránh được một số cơ chế phát hiện lỗi thời.',
        zn_linker: 'Sử dụng trình liên kết Zygisk Next (Thử nghiệm)',
        zn_linker_desc: 'Hãy sử dụng trình liên kết được tích hợp sẵn thay vì trình liên kết hệ thống để nạp mô-đun. Điều này sẽ tăng cường khả năng tàng hình nhưng có thể gây ra các vấn đề về khả năng tương thích.',
    },
    zygote_inject_state: {
        running: 'Đang chạy',
        stop_by_user: 'Dừng lại bởi người dùng',
        stop_by_crash: 'Dừng lại bởi lỗi Zygote',
        running_desc: 'Trình giám sát Zygote đang chạy bình thường.',
        stop_by_user_desc: 'Trình giám sát Zygote bị dừng lại bởi người dùng.',
        stop_by_crash_desc: 'Nhiều lần khởi động lại của Zygote đã được phát hiện, Trình giám sát Zygote đã tự động dừng lại.',
    },
    zygote_state: {
        unknown: 'Không xác định',
        injected: 'Đã tiêm ({pid})',
        inject_failed: 'Tiêm thất bại ({pid})',
        skipped: 'Đã bỏ qua ({pid})',
        unknown_desc: 'Trạng thái của Zygote này là không xác định và có thể tồn tại nhưng chưa bắt đầu, hoặc Trình giám sát Zygote không phát hiện được việc bắt đầu của nó.',
        injected_desc: 'Zygisk đã được tiêm vào Zygote. ID tiến trình của nó là {pid}',
        inject_failed_desc: 'Zygisk đã thử để được tiêm vào Zygote này đã nhưng thất bại. ID tiến trình của nó là {pid}',
        skipped_desc: 'Zygote đã được giám sát để khởi động. ID tiến trình của nó là {pid}. Nhưng việc tiêm Zygisk đã dừng lại do nhiều lần khởi động lại mềm trước đó của hệ thống.'
    },
    corrupted: {
        title: 'Tệp mô-đun bị hỏng',
        desc: 'Vui lòng hoàn tác các thay đổi lên Zygisk Next và thử lại.'
    }
}
