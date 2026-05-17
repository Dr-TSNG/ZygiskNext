export default {
    language: { 
        name: 'العربية'
    },
    main_fragment: {
        dashboard: 'الحالة',
        basic: 'معلومات أساسية',
        settings: 'الإعدادات'
    },
    dashboard: {
        root_impl: 'تنفيذ الروت',
        zygote_monitor: 'مراقب Zygote',
        zygisk_module_title: 'لا توجد وحدات Zygisk | وحدة Zygisk ({0}) | وحدات Zygisk ({0})',
        zn_module_title: 'لا توجد وحدات ZN | وحدة ZN ({0}) | وحدات ZN ({0})',
        root_impl_normal: 'تنفيذ الروت الحالي هو {impl}، قائمة الحظر تعمل بشكل صحيح.',
        root_impl_abnormal: 'تعذر تحديد تنفيذ الروت، قائمة الحظر لن تعمل.',
        root_impl_multiple: 'تم العثور على أكثر من تنفيذ روت، قائمة الحظر لن تعمل.',
        kernelsu_denylist: 'قائمة الحظر في KernelSU تشير إلى التطبيقات المعلّمة بـ "Unmount modules" في ملف التطبيق.',
        magisk_denylist: 'قائمة الحظر في Magisk تشير إلى قائمة الحظر المدمجة في Magisk.',
        apatch_denylist: 'قائمة الحظر في APatch تشير إلى تطبيق SuperUser حيث يكون خيار "Exclude" مفعّلًا ويتم تعطيل الروت.',
    },
    settings: {
        log_to_kernel: 'تسجيل السجلات في dmesg (للمطورين فقط)',
        nonroot_as_denylist: 'اعتبار التطبيقات بدون روت ضمن قائمة الحظر',
        enforce_denylist: 'سياسة قائمة الحظر',
        enforce_denylist_desc: 'مفعل: منع حقن الكود وإرجاع تعديلات الربط للتطبيقات في قائمة الحظر.<br/>فك الربط فقط: إرجاع تعديلات الربط مع السماح بحقن الكود للتطبيقات في قائمة الحظر.',
        enforce_denylist_alert: 'للمستخدمين العاديين، يُنصح بشدة تعطيل ميزة KernelSU umount يدويًا من مدير KernelSU لتجنب فك نقاط الربط عدة مرات بسبب سوء الإعداد.',
        denylist_disabled: 'معطلة',
        denylist_enforced: 'مفعلة',
        denylist_just_umount: 'فك الربط فقط',
        anonymous_memory: 'استخدام ذاكرة مجهولة',
        anonymous_memory_desc: 'تحميل الوحدات في ذاكرة مجهولة. هذا يقلل من وضوح السجلات لكنه يتجاوز بعض آليات الكشف القديمة.',
        zn_linker: 'استخدام رابط Zygisk Next',
        zn_linker_desc: 'استخدام الرابط المدمج بدل رابط النظام لتحميل الوحدات. هذا يزيد التخفي لكنه قد يسبب مشاكل توافق.',
    },
    zygote_inject_state: {
        running: 'يعمل',
        stop_by_user: 'متوقف بواسطة المستخدم',
        stop_by_crash: 'متوقف بسبب تعطل zygote',
        running_desc: 'مراقب Zygote يعمل بشكل طبيعي.',
        stop_by_user_desc: 'تم إيقاف مراقب Zygote بواسطة المستخدم.',
        stop_by_crash_desc: 'تم اكتشاف إعادة تشغيل متكررة لـ Zygote، وتم إيقاف المراقب تلقائيًا.',
    },
    zygote_state: {
        unknown: 'غير معروف',
        injected: 'تم الحقن ({pid})',
        inject_failed: 'فشل الحقن ({pid})',
        skipped: 'تم التجاوز ({pid})',
        unknown_desc: 'حالة Zygote غير معروفة، قد يكون موجودًا لكنه لم يبدأ، أو أن المراقب لم يكتشف بدءه.',
        injected_desc: 'تم حقن Zygisk في Zygote. رقم العملية هو {pid}',
        inject_failed_desc: 'تمت محاولة حقن Zygisk في Zygote لكنها فشلت. رقم العملية هو {pid}',
        skipped_desc: 'تمت مراقبة بدء Zygote. رقم العملية هو {pid}. لكن تم إيقاف الحقن بسبب عدة عمليات إعادة تشغيل ناعمة سابقة للنظام.'
    },
    corrupted: {
        title: 'ملفات الوحدة تالفة',
        desc: 'يرجى التراجع عن التغييرات في Zygisk Next والمحاولة مرة أخرى.'
    },
    module: {
        issue: {
            title: 'هذه الوحدة بها مشكلة',
            companion_api_issue: 'هذه الوحدة {name} بها مشكلة في الاستخدام غير الصحيح لـ API الرفيق، مما قد يسبب أعطال العملية وتسريب الذاكرة. يرجى الاتصال بمطور هذه الوحدة لحل المشكلة.',
            learn_more: 'زيارة هنا للحصول على مزيد من المعلومات：{link}',
            check_banner: 'لم يتم اكتشاف وحدات بها مشكلات. | تم اكتشاف وحدة بها مشكلات ({0}). يرجى مراجعة قائمة الوحدات. | تم اكتشاف {0} وحدات بها مشكلات. يرجى مراجعة قائمة الوحدات.',
            badge: 'مشكلة',
        },
        zn: {
            process_count: 'لا توجد عمليات | عملية واحدة ({0}) | {0} عمليات',
        },
    }
}
