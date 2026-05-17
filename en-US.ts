export default {
    language: { 
        name: 'English (US)'
    },
    main_fragment: {
        dashboard: 'Status',
        basic: 'Basic Information',
        settings: 'Settings'
    },
    dashboard: {
        root_impl: 'Root implementation',
        zygote_monitor: 'Zygote Monitor',
        zygisk_module_title: 'No Zygisk modules | Zygisk module ({0}) | Zygisk modules ({0})',
        zn_module_title: 'No ZN modules | ZN module ({0}) | ZN modules ({0})',
        root_impl_normal: 'Current Root Implementation is {impl}, denylist will work properly.',
        root_impl_abnormal: 'Could not determine Root Implementation, denylist will not work.',
        root_impl_multiple: 'Multiple Root Implementations found, denylist will not work.',
        kernelsu_denylist: 'Denylist of KernelSU refers to apps that are marked as \'Unmount modules\' in App Profile.',
        magisk_denylist: 'Denylist of Magisk refers to Magisk\'s built-in denylist.',
        apatch_denylist: 'Denylist of APatch refers to to SuperUser App which \'Exclude\' is enabled and disable root',
    },
    settings: {
        log_to_kernel: 'Log to dmesg (Only for developers)',
        nonroot_as_denylist: 'Treat non-root apps as denylist',
        enforce_denylist: 'Denylist Policy',
        enforce_denylist_desc: 'Enforced: Block code injection and revert mount modifications for apps in denylist.<br/>Umount Only: Revert mount modifications but allow code injection for apps in denylist.',
        enforce_denylist_alert: 'For normal users, it is strongly recommended to manually disable kernel umount feature in KernelSU manager to avoid mount points being unmounted multiple times due to misconfiguration.',
        denylist_disabled: 'Disabled',
        denylist_enforced: 'Enforced',
        denylist_just_umount: 'Unmount Only',
        anonymous_memory: 'Use anonymous memory',
        anonymous_memory_desc: 'Load modules into anonymous memory. This compromises log readability but circumvents certain outdated detection mechanisms.',
        zn_linker: 'Use Zygisk Next linker',
        zn_linker_desc: 'Use built-in linker instead of the system linker to load modules. This will enhance stealth but may cause compatibility issues.',
    },
    zygote_inject_state: {
        running: 'Running',
        stop_by_user: 'Stop by user',
        stop_by_crash: 'Stop by zygote crashed',
        running_desc: 'Zygote Monitor is running normally.',
        stop_by_user_desc: 'Zygote Monitor stopped by user.',
        stop_by_crash_desc: 'Repeated restarts of Zygote has been detected, Zygote Monitor has automatically stopped.',
    },
    zygote_state: {
        unknown: 'Unknown',
        injected: 'Injected ({pid})',
        inject_failed: 'Inject failed ({pid})',
        skipped: 'Skipped ({pid})',
        unknown_desc: 'The status of this Zygote is unknown and may exist but not be started, or the Zygote monitor may not detect its start.',
        injected_desc: 'Zygisk has been injected into the Zygote. Its process ID is {pid}',
        inject_failed_desc: 'Zygisk was attempted to be injected into this Zygote but failed. Its process ID is {pid}',
        skipped_desc: 'The Zygote was monitored to start. Its process ID is {pid}. But Zygisk injecting was stopped due to multiple previous soft reboots of the system.'
    },
    corrupted: {
        title: 'Module files corrupted',
        desc: 'Please revert the changes to Zygisk Next and try again.'
    },
    module: {
        issue: {
            title: 'This module has an issue',
            companion_api_issue: 'This module {name} has an issue with improper use of the Companion API, which may cause process crashes and memory leaks. Please contact the developer of this module to resolve the issue.',
            learn_more: 'Visit here for more information: {link}',
            check_banner: 'No problematic modules detected. | {0} module has an issue. Please review the module list. | {0} modules have issues. Please review the module list.',
            badge: 'Issue',
        },
        zn: {
            process_count: 'No process | {0} process | {0} processes',
        },
    }
}
