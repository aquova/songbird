import sys

class ANSI:
    RED = '\033[0;31m'
    YELLOW = '\033[0;33m'
    GREEN = '\033[0;32m'
    BLUE = '\033[0;36m'
    CLEAR = '\033[0m'

CART_TYPE_STR = {
    0x00: "ROM ONLY",
    0x01: "MBC1",
    0x02: "MBC1 + RAM",
    0x03: "MBC1 + RAM + BATTERY",
    0x05: "MBC2",
    0x06: "MBC2 + BATTERY",
    0x08: "ROM + RAM",
    0x09: "ROM + RAM + BATTERY",
    0x0b: "MMM01",
    0x0c: "MMM01 + RAM",
    0x0d: "MMM01 + RAM + BATTERY",
    0x0f: "MBC3 + TIMER + BATTERY",
    0x10: "MBC3 + TIMER + RAM + BATTERY",
    0x11: "MBC3",
    0x12: "MBC3 + RAM",
    0x13: "MBC3 + RAM + BATTERY",
    0x15: "MBC4",
    0x16: "MBC4 + RAM",
    0x17: "MBC4 + RAM + BATTERY",
    0x19: "MBC5",
    0x1a: "MBC5 + RAM",
    0x1b: "MBC5 + RAM + BATTERY",
    0x1c: "MBC5 + RUMBLE",
    0x1d: "MBC5 + RUMBLE + RAM",
    0x1e: "MBC5 + RUMBLE + RAM + BATTERY",
    0xfc: "POCKET CAMERA",
    0xfd: "BANDAI TAMA5",
    0xfe: "HuC3",
    0xff: "HuC1 + RAM + BATTERY"
}

ROM_SIZE_STR = {
    0x00: "32 KiB - 2 banks, no switching",
    0x01: "64 KiB - 4 banks",
    0x02: "128 KiB - 8 banks",
    0x03: "256 KiB - 16 banks",
    0x04: "512 KiB - 32 banks",
    0x05: "1 MiB - 64 banks",
    0x06: "2 MiB - 128 banks",
    0x07: "4 MiB - 256 banks",
    0x52: "1.1 MiB - 72 banks",
    0x53: "1.2 MiB - 80 banks",
    0x54: "1.5 MiB - 96 banks"
}

RAM_SIZE_STR = {
    0x00: "No Cartridge RAM",
    0x01: "2 KiB",
    0x02: "8 KiB",
    0x03: "32 KiB"
}

def main():
    with open(sys.argv[1], "rb") as file:
        rom = file.read()
        cgb_support = (rom[0x143] == 0x80)
        cgb_only = (rom[0x143] == 0xc0)
        sgb = (rom[0x146] == 0x03)

        title = ""
        manu_code = None
        if cgb_support or cgb_only:
            title = rom[0x134:0x13f]
            manu_code = rom[0x13f:0x143]
        else:
            title = rom[0x134:0x143]

        licensee_code = rom[0x14b]
        if rom[0x14b] == 0x33:
            licensee_code = rom[0x144:0x146].decode('utf-8')

        cart_type = rom[0x147]
        rom_size = rom[0x148]
        ram_size = rom[0x149]

        jp_dest = "JP" if rom[0x14a] == 0x00 else "Non-JP"
        rom_version = rom[0x14c]
        header_checksum = rom[0x14d]
        rom_checksum_raw = rom[0x14e:0x150]
        rom_checksum = rom_checksum_raw[0] << 8 | rom_checksum_raw[1]

        system_str = "Game Boy"
        if cgb_support:
            system_str += " / Game Boy Color"
        elif cgb_only:
            system_str = "Game Boy Color"

        if sgb:
            system_str += " w/ Super Game Boy support"

        print(f"{ANSI.RED}Game Boy Header Inspector{ANSI.CLEAR}\n")
        print(f"{ANSI.BLUE}Title: \t\t\t{ANSI.CLEAR}{title.decode('utf-8')}")
        print(f"{ANSI.BLUE}System: \t\t{ANSI.GREEN}{system_str}{ANSI.CLEAR}")
        print("")
        print(f"{ANSI.BLUE}Cartridge Type: \t{ANSI.CLEAR}{CART_TYPE_STR[cart_type]}")
        print(f"{ANSI.BLUE}ROM Size: \t\t{ANSI.CLEAR}{ROM_SIZE_STR[rom_size]}")
        print(f"{ANSI.BLUE}RAM Size: \t\t{ANSI.CLEAR}{RAM_SIZE_STR[ram_size]}")
        print("")
        print(f"{ANSI.BLUE}Destination: \t\t{ANSI.CLEAR}{jp_dest}")
        if manu_code != None:
            print(f"{ANSI.BLUE}Manufacturer Code: {ANSI.CLEAR}{manu_code.decode('utf-8')}")
        print(f"{ANSI.BLUE}Licensee Code: \t\t{ANSI.CLEAR}{licensee_code}")
        print(f"{ANSI.BLUE}ROM Version: \t\t{ANSI.CLEAR}{rom_version}")
        print("")
        print(f"{ANSI.BLUE}Header Checksum: \t{ANSI.CLEAR}${header_checksum:x}")
        print(f"{ANSI.BLUE}ROM Checksum: \t\t{ANSI.CLEAR}${rom_checksum:x}")

if __name__ == "__main__":
    main()
