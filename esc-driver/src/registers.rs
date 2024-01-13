use super::*;
use crate::enums::*;

implement_registers!(
    impl Esc {
        /// Number of supported Sync Manager Channels
        ///
        /// Physical address: 0x0005
        /// Possible values: 0x00 ..= 0x10
        register NoOfSuppSyncManChannels {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0005;
            const SIZE_BITS : usize = 8;
            const RESET_VALUE : [u8] = [0];

            value : u8 = 0 .. 16,
        },

        /// RAM size in KiB (Kibibyte = 1024 Bytes)
        ///
        /// Physical address: 0x0006
        /// Possible values: 1..=60
        register RamSize {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0006;
            const SIZE_BITS : usize = 8;
            const RESET_VALUE : [u8] = [0];

            value : u8 = 0 .. 16,
        },

        /// AL Control
        ///
        /// Physical address: 0x0120
        register AlControl {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0120;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [1, 0];

            /// State
            state : u8 as EsmState = 0 .. 4,
            /// Acknowledge
            acknowledge : bool = 4,
            /// IdRequest
            id_request : bool = 5,
        },

        /// AL Status
        ///
        /// Physical address: 0x0130 ..= 0x0131
        register AlStatus {
            type RWType = ReadWrite;
            const ADDRESS : u16 = 0x0130;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [1, 0];

            /// State
            state : u8 as EsmState = 0 .. 4,
            /// Change
            change : bool = 4,
            /// IdLoaded
            id_loaded : bool = 5,
        },

        /// AL Status Code
        ///
        /// Physical address: 0x0134 ..= 0x0135
        register AlStatusCode {
            type RWType = ReadWrite;
            const ADDRESS : u16 = 0x0134;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [0, 0];

            value : u16 as enums::AlStatusCode = 0 .. 16,
        },

        /// AL Event mask
        ///
        /// Physical address: 0x0204 ..= 0x0207
        register AlEventMask {
            type RWType = ReadWrite;
            const ADDRESS : u16 = 0x0204;
            const SIZE_BITS : usize = 32;
            const RESET_VALUE : [u8] = [0, 0, 0, 0];

            al_control_change : bool = 0,
            dc_event_0 : bool = 1,
            dc_event_1 : bool = 2,
            dc_event_2 : bool = 3,
            sync_manager_change : bool = 4,
            eeprom_emulation : bool = 5,
            sync_manager_channel_0 : bool = 8,
            sync_manager_channel_1 : bool = 9,
            sync_manager_channel_2 : bool = 10,
            sync_manager_channel_3 : bool = 11,
            sync_manager_channel_4 : bool = 12,
            sync_manager_channel_5 : bool = 13,
            sync_manager_channel_6 : bool = 14,
            sync_manager_channel_7 : bool = 15,
            sync_manager_channel_8 : bool = 16,
            sync_manager_channel_9 : bool = 17,
            sync_manager_channel_10 : bool = 18,
            sync_manager_channel_11 : bool = 19,
            sync_manager_channel_12 : bool = 20,
            sync_manager_channel_13 : bool = 21,
            sync_manager_channel_14 : bool = 22,
            sync_manager_channel_15 : bool = 23,
        },

        /// AL Event
        ///
        /// Physical address: 0x0220 ..= 0x0223
        register AlEvent {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0220;
            const SIZE_BITS : usize = 32;
            const RESET_VALUE : [u8] = [0, 0, 0, 0];

            al_control_change : bool = 0,
            dc_event_0 : bool = 1,
            dc_event_1 : bool = 2,
            dc_event_2 : bool = 3,
            sync_manager_change : bool = 4,
            eeprom_emulation : bool = 5,
            sync_manager_channel_0 : bool = 8,
            sync_manager_channel_1 : bool = 9,
            sync_manager_channel_2 : bool = 10,
            sync_manager_channel_3 : bool = 11,
            sync_manager_channel_4 : bool = 12,
            sync_manager_channel_5 : bool = 13,
            sync_manager_channel_6 : bool = 14,
            sync_manager_channel_7 : bool = 15,
            sync_manager_channel_8 : bool = 16,
            sync_manager_channel_9 : bool = 17,
            sync_manager_channel_10 : bool = 18,
            sync_manager_channel_11 : bool = 19,
            sync_manager_channel_12 : bool = 20,
            sync_manager_channel_13 : bool = 21,
            sync_manager_channel_14 : bool = 22,
            sync_manager_channel_15 : bool = 23,
        },

        /// Watchdog divider in 40ns
        ///
        /// Physical address: 0x0400 ..= 0x0401
        register WatchdogDivider {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0400;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [0xC2, 0x09];

            value : u16 = 0 .. 16,
        },

        /// Sync manager watchdog
        ///
        /// Physical address: 0x0420 ..= 0x0421
        register SyncManChannelWatchdog {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0420;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [0xE8, 0x03];

            value : u16 = 0 .. 16,
        },

        /// Sync manager channel watchdog status
        ///
        /// Physical address: 0x0440 ..= 0x0441
        register SyncManChannelWatchdogStatus {
            type RWType = ReadOnly;
            const ADDRESS : u16 = 0x0440;
            const SIZE_BITS : usize = 16;
            const RESET_VALUE : [u8] = [0, 0];

            value : bool = 0,
        },
    }
);
