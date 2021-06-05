pub enum IoPortStart {
    DMA_LEGECY = 0x0000, // 0x001F 	The first legacy DMA controller, often used for transfers to floppies.
    PIC1 = 0x0020,       // 0x0021 	The first Programmable Interrupt Controller
    MSR = 0x0022,        //-0x0023 	Access to the Model-Specific Registers of Cyrix processors.
    PIT = 0x0040,        // 0x0047 	The PIT (Programmable Interval Timer)
    PS2 = 0x0060, // 0x0064 	The "8042" PS/2 Controller or its predecessors, dealing with keyboards and mice.
    CMOS = 0x0070, // 0x0071 	The CMOS and RTC registers
    DMA = 0x0080, // 0x008F 	The DMA (Page registers)
    A20 = 0x0092, //	The location of the fast A20 gate register
    PIC2 = 0x00A0, // 0x00A1 	The second PIC
    DMA2 = 0x00C0, // 0x00DF 	The second DMA controller, often used for soundblasters
    E9_HACK = 0x00E9, //	Home of the Port E9 Hack. Used on some emulators to directly send text to the hosts' console.
    ATA_CONTROL1 = 0x0170, // 0x0177 	The secondary ATA harddisk controller.
    ATA_CONTROL2 = 0x01F0, // 0x01F7 	The primary ATA harddisk controller.
    PARRALLE = 0x0278, // 0x027A 	Parallel port
    SERIAL2 = 0x02F8, // 0x02FF 	Second serial port
    IMB_VGA = 0x03B0, // 0x03DF 	The range used for the IBM VGA, its direct predecessors, as well as any modern video card in legacy mode.
    FDC = 0x03F0,     // 0x03F7 	Floppy disk controller
    SERIAL1 = 0x03F8, // 0x03FF 	First serial port
}
