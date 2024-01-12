# Milestones

## catling-poc
### Description
[catling-poc](#catling-poc) is a proof of concept for a Rust based *async* EtherCAT Subdevice software stack.
It should run together with the following hardware
- STM NUCLEO-F439ZI
- Beckhoff FB1111-0141 (SPI interface)

To connect both an adapting pcb is needed.
'ToDo: Link to github repository'

The SPI interface has the following benefits:
- Widely available
- Supporting async HALs are available for rust in combination with the used STM microcontroller

### Features
The poc should have the following features:
- [ ] Mailbox support
- [ ] CoE support (only default objects)
- [ ] FreeRun

### Design
The poc will be designed with the following crates:
- RTIC v2 as main runtime
- embassy-stm as HAL

