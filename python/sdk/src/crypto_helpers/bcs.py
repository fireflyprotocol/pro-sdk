import struct


class BCSSerializer:
    def __init__(self):
        self.buffer = bytearray()

    def serialize_bool(self, value: bool):
        if not isinstance(value, bool):
            raise TypeError("Expected a boolean value.")
        self.buffer.append(1 if value else 0)

    def serialize_u8(self, value: int):
        self._serialize_integer(value, 1, "B")

    def serialize_u16(self, value: int):
        self._serialize_integer(value, 2, "H")

    def serialize_u32(self, value: int):
        self._serialize_integer(value, 4, "I")

    def serialize_u64(self, value: int):
        self._serialize_integer(value, 8, "Q")

    def serialize_u128(self, value: int):
        if not (0 <= value < 2**128):
            raise ValueError("Value out of range for u128.")
        low = value & 0xFFFFFFFFFFFFFFFF
        high = value >> 64
        self.serialize_u64(low)
        self.serialize_u64(high)


    def serialize_str(self, value: str):
        if not isinstance(value, str):
            raise TypeError("Expected a string.")
        encoded = value.encode("utf-8")
        self.serialize_u8(len(encoded))  # Use u8 (1 byte) for the length
        self.buffer.extend(encoded)

    def serialize_list(self, values: list, element_serializer):
        self.serialize_u32(len(values))  # Length prefix
        for value in values:
            element_serializer(value)

    def serialize_tuple(self, values: tuple, element_serializers):
        if len(values) != len(element_serializers):
            raise ValueError(
                "Tuple length must match the number of serializers.")
        for value, serializer in zip(values, element_serializers):
            serializer(value)

    def serialize_dict(self, dictionary: dict, key_serializer, value_serializer):
        self.serialize_u32(len(dictionary))  # Length prefix
        for key, value in dictionary.items():
            key_serializer(key)
            value_serializer(value)

    def serialize_address(self, address: str):
        """
        Serializes an Address (fixed-size byte array).
        For example, in Diem/Aptos, an Address is a 16-byte or 32-byte identifier.
        """

        address = hex_to_byte_array(address)

        if not isinstance(address, (bytes, bytearray)):
            raise TypeError("Address must be a byte array.")

        if len(address) not in {16, 32}:
            raise ValueError("Address must be 16 or 32 bytes in length.")
        self.buffer.extend(address)

    def serialize_uint8_array(self, array: list):
        """
        Serializes an array of uint8 values with a compact length prefix (u8).

        Args:
            array (list): List of integers (0-255) representing uint8 values.
        """
        if not isinstance(array, list):
            raise TypeError("Expected a list of uint8 values.")
        if any(not (0 <= value < 256) for value in array):
            raise ValueError(
                "All elements in the array must be in the range 0-255.")

        self.buffer.extend(decimal_to_bcs(len(array)))  # Serialize length as a single byte (u8)
        self.buffer.extend(array)     # Append raw uint8 values

    def _serialize_integer(self, value: int, byte_size: int, format_char: str):
        if not (0 <= value < 2**(byte_size * 8)):
            raise ValueError(
                f"Value out of range for {byte_size * 8}-bit integer.")
        # Little-endian format
        self.buffer.extend(struct.pack("<" + format_char, value))

    def get_bytes(self):
        return bytes(self.buffer)


def hex_to_byte_array(hex_address: str) -> bytearray:
    """
    Converts a hexadecimal address string to a byte array.

    Args:
        hex_address (str): The hexadecimal string (e.g., '1a2b3c4d...').

    Returns:
        bytearray: The corresponding byte array.
    """
    if not isinstance(hex_address, str):
        raise TypeError("Hex address must be a string.")

    # Remove optional prefix "0x" if present
    if hex_address.startswith("0x") or hex_address.startswith("0X"):
        hex_address = hex_address[2:]

    # Ensure the hex string has an even length
    if len(hex_address) % 2 != 0:
        raise ValueError("Hex address must have an even number of characters.")

    try:
        return bytearray.fromhex(hex_address)
    except ValueError as e:
        raise ValueError(f"Invalid hex string: {e}")


def decimal_to_bcs(num):
        # Initialize an empty list to store the BCS bytes
        bcs_bytes = []
        while num > 0:
            # Take the last 7 bits of the number
            bcs_byte = num & 0x7F

            # Set the most significant bit (MSB) to 1 if there are more bytes to follow
            if num > 0x7F:
                bcs_byte |= 0x80

            # Append the BCS byte to the list
            bcs_bytes.append(bcs_byte)

            # Right-shift the number by 7 bits to process the next portion
            num >>= 7

        return bcs_bytes