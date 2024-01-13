use crate::*;

#[derive(Debug, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum EsmState {
    /// Init
    Init = 1,
    /// Pre-Operational
    PreOperational = 2,
    ///Bootstrap
    Bootstrap = 3,
    /// Safe-Operational
    SafeOperational = 4,
    ///Operational
    Operational = 8,
}

#[derive(Debug, Copy, Clone, Format, FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum AlStatusCode {
    /// No error
    NoError = 0x0000,
    /// Unspecified error
    UnspecifiedError = 0x0001,
    /// No Memory
    NoMemory = 0x0002,
    /// Invalid Device Setup
    InvalidDeviceSetup = 0x0003,
    /// Invalid requested state change
    InvalidRequestedStateChange = 0x0011,
    /// Unknown requested state
    UnknownRequestedState = 0x0012,
    /// Bootstrap not supported
    BootstrapNotSupported = 0x0013,
    /// No valid firmware
    NoValidFirmware = 0x0014,
    /// Invalid mailbox configuration Init to Bootstrap state change
    InvalidMailboxConfigurationInitBootstrap = 0x0015,
    /// Invalid mailbox configuration Init to PreOperational state change
    InvalidMailboxConfigurationInitPreOperational = 0x0016,
    /// Invalid sync manager configuration
    InvalidSyncManagerConfiguration = 0x0017,
    /// No valid inputs available
    NoValidInputsAvailable = 0x0018,
    /// No valid outputs
    NoValidOutputs = 0x0019,
    /// Synchronization error
    SynchronizationError = 0x001A,
    /// Sync manager watchdog
    SyncManagerWatchdog = 0x001B,
    /// Invalid Sync Manager Types
    InvalidSyncManagerTypes = 0x001C,
    /// Invalid Output Configuration
    InvalidOutputConfiguration = 0x001D,
    /// Invalid Input Configuration
    InvalidInputConfiguration = 0x001E,
    /// Invalid Watchdog Configuration
    InvalidWatchdogConfiguration = 0x001F,
    /// Slave needs cold start
    SlaveNeedsColdStart = 0x0020,
    /// Slave needs INIT
    SlaveNeedsInit = 0x0021,
    /// Slave needs PREOP
    SlaveNeedsPreOp = 0x0022,
    /// Slave needs SAFEOP
    SlaveNeedsSafeOp = 0x0023,
    /// Invalid Input Mapping
    InvalidInputMapping = 0x0024,
    /// Invalid Output Mapping
    InvalidOutputMapping = 0x0025,
    /// Inconsistent Settings
    InconsistentSettings = 0x0026,
    /// FreeRun not supported
    FreeRunNotSupported = 0x0027,
    /// SyncMode not supported
    SyncModeNotSupported = 0x0028,
    /// FreeRun needs 3Buffer Mode
    FreeRunNeeds3BufferMode = 0x0029,
    /// Background Watchdog
    BackgroundWatchdog = 0x002A,
    /// No Valid Inputs and Outputs
    NoValidInputsAndOutputs = 0x002B,
    /// Fatal Sync Error
    FatalSyncError = 0x002C,
    /// No Sync Error
    NoSyncError = 0x002D,
    /// Invalid DC SYNC Configuration
    InvalidDcSyncConfiguration = 0x0030,
    /// Invalid DC Latch Configuration
    InvalidDcLatchConfiguration = 0x0031,
    /// PLL Error
    PllError = 0x0032,
    /// DC Sync IO Error
    DcSyncIoError = 0x0033,
    /// DC Sync Timeout Error
    DcSyncTimeoutError = 0x0034,
    /// DC Invalid Sync Cycle Time
    DcInvalidSyncCycleTime = 0x0035,
    /// DC Sync0 Cycle Time
    DcSync0CycleTime = 0x0036,
    /// DC Sync1 Cycle Time
    DcSync1CycleTime = 0x0037,
    /// MBX_AOE
    MbxAoe = 0x0041,
    /// MBX_EOE
    MbxEoe = 0x0042,
    /// MBX_COE
    MbxCoe = 0x0043,
    /// MBX_FOE
    MbxFoe = 0x0044,
    /// MBX_SOE
    MbxSoe = 0x0045,
    /// MBX_VOE
    MbxVoe = 0x004F,
    /// EEPROM no access
    EepromNoAccess = 0x0050,
    /// EEPROM Error
    EepromError = 0x0051,
    /// Slave restarted locally
    SlaveRestartedLocally = 0x0060,
    /// Device Identification value updated
    DeviceIdentificationValueUpdated = 0x0061,
    /// Application controller available
    ApplicationControllerAvailable = 0x00F0,
    /// Unknown
    #[num_enum(catch_all)]
    Unknown(u16),
}
