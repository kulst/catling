/* Linker script for the STM32F439ZITx */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 2048K

    /* .bss, .data and the heap go in this region */
    RAM : ORIGIN = 0x20000000, LENGTH = 192K

    /* CCRAM : ORIGIN = 0x10000000, LENGTH = 8K */
}

/*_stack_start = ORIGIN(CCRAM) + LENGTH(CCRAM); */
