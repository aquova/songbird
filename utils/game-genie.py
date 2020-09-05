# Convert Game Boy Game Genie codes into RAM address/value and back
# Written by aquova, 2020

# Reference for Game Genie decoding: https://www.youtube.com/watch?v=C86OsYRACTM

import argparse

ENCODED_LEN = 6
ENCODED_LEN_W_CHECK = 9

def setup_args():
    parser = argparse.ArgumentParser(description="Encode and decode Game Genie codes")
    parser.add_argument('-d', dest="decode", action="store_true", help="Should input be decoded back to a RAM address")
    parser.add_argument('code', type=str, help="Input to encode or decode")
    parser.set_defaults(decode=False)

    return parser.parse_args()

def decode(code):
    # Disregard any separating hyphens, if any
    encoded = code.replace("-", "")

    # Codes can only be 6 or 9 characters
    if len(encoded) != ENCODED_LEN and len(encoded) != ENCODED_LEN_W_CHECK:
        print("This code is not the correct size to be a Game Genie code")
        return

    # Make sure this is a hex string
    try:
        int(encoded, 16)
    except ValueError:
        print("This code contains invalid characters (only 0-9, A-F)")
        return

    # First two digits are simply the value to replace
    val = encoded[0:2]
    addr1 = int(encoded[5], 16)
    addr = hex(~addr1 % 0x10) + encoded[2:5]

    check = ""
    if len(encoded) == ENCODED_LEN_W_CHECK:
        # Integrity check, the 7th and 8th digits must be the same if you flip the MSB of digit 8
        mask = 0b1000
        seven = int(encoded[6], 16)
        eight = int(encoded[7], 16)
        integrity = eight ^ mask
        if seven != integrity:
            print("The integrity check has failed. This is not a valid code.")
            return

        # Check is annoying. It takes the 7th and 9th bytes, like so: GGgg HHhh
        # And rearranges them to be ~hh~G~G ~gg~HH
        nine = int(encoded[8], 16)

        # Flip necessary bits, break into four pieces of 2 bits each
        GGgg = (seven ^ 0b1110)
        HHhh = (nine ^ 0b1010)
        GG = (GGgg & 0b1100) >> 2
        gg = (GGgg & 0b0011)
        HH = (HHhh & 0b1100) >> 2
        hh = (HHhh & 0b0011)

        # Stitch back together
        verify = (hh << 6) | (GG << 4) | (gg << 2) | HH
        check = f" if the original byte is {hex(verify)}"

    print(f"Game Genie code {code} will set {addr} to always be 0x{val}{check}")

def encode(code):
    try:
        parts = code.split(":")
        val = parts[1]
        data = parts[0].split("?")
        addr = data[0]
        check = None
        if len(data) > 1:
            check = data[1]
    except IndexError:
        print("To generate a code, specify a RAM address and value to replace as ADDR:VAL. If you wish to only overwrite a certain byte value, specify as ADDR?CHECK:VAL.")
        return

    six = int(addr[0], 16)
    six = ~six % 0x10

    digits = val + addr[1:] + hex(six)[2:]
    output = digits[0:3] + "-" + digits[3:]

    if check != None:
        verify = int(check, 16)
        hh = ((verify ^ 0b10000000) & 0b11000000) >> 6
        GG = ((verify ^ 0b00110000) & 0b00110000) >> 4
        gg = ((verify ^ 0b00001000) & 0b00001100) >> 2
        HH = ((verify ^ 0b00000010) & 0b00000011)

        digit1 = GG << 2 | gg
        digit2 = digit1 ^ 0b1000
        digit3 = HH << 2 | hh

        digits = hex(digit1)[2:] + hex(digit2)[2:] + hex(digit3)[2:]
        output += "-" + digits

    print(output.upper())

def main():
    args = setup_args()

    if args.decode:
        decode(args.code)
    else:
        encode(args.code)

if __name__ == "__main__":
    main()
