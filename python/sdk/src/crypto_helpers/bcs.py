import struct


class BCSSerializer:
    """
    Binary Canonical Serialization (BCS) implementation.
    Matches Rust's bcs crate serialization format.
    """

    def __init__(self):
        self.buffer = bytearray()
        self.MAX_SEQUENCE_LENGTH = 0xFFFFFFFF  # u32::MAX

    def serialize_bool(self, value: bool) -> None:
        if not isinstance(value, bool):
            raise TypeError("Expected a boolean value.")
        self.buffer.append(1 if value else 0)

    def serialize_u8(self, value: int) -> None:
        """
        Serialize an unsigned 8-bit integer.
        
        Args:
            value: Integer in range 0-255
        Raises:
            ValueError: If value outside uint8 range
        """
        self._serialize_integer(value, 1, "B")

    def serialize_u16(self, value: int) -> None:
        """
        Serialize an unsigned 16-bit integer.
        
        Args:
            value: Integer in range 0-65535
        Raises:
            ValueError: If value outside uint16 range
        """
        self._serialize_integer(value, 2, "H")

    def serialize_u32(self, value: int) -> None:
        """
        Serialize an unsigned 32-bit integer.
        
        Args:
            value: Integer in range 0-4294967295
        Raises:
            ValueError: If value outside uint32 range
        """
        self._serialize_integer(value, 4, "I")

    def serialize_u64(self, value: int) -> None:
        self._serialize_integer(value, 8, "Q")

    def serialize_u128(self, value: int) -> None:
        if not (0 <= value < 2**128):
            raise ValueError("Value out of range for u128.")
        low = value & 0xFFFFFFFFFFFFFFFF
        high = value >> 64
        self.serialize_u64(low)
        self.serialize_u64(high)


    def serialize_str(self, value: str) -> None:
        if not isinstance(value, str):
            raise TypeError("Expected a string.")
        encoded = value.encode("utf-8")
        if len(encoded) > self.MAX_SEQUENCE_LENGTH:
            raise ValueError(f"String length {len(encoded)} exceeds maximum {self.MAX_SEQUENCE_LENGTH}")
        self.serialize_u32(len(encoded))  # Change from u8 to u32
        self.buffer.extend(encoded)

    def serialize_list(self, values: list, element_serializer: callable) -> None:
        """
        Serialize a list with u32 length prefix.
        
        Args:
            values: List of elements to serialize
            element_serializer: Function to serialize each element
        Raises:
            ValueError: If length exceeds MAX_SEQUENCE_LENGTH
        """
        if len(values) > self.MAX_SEQUENCE_LENGTH:
            raise ValueError(f"List length {len(values)} exceeds maximum {self.MAX_SEQUENCE_LENGTH}")
        self.serialize_u32(len(values))  # Length prefix
        for value in values:
            element_serializer(value)

    def serialize_tuple(self, values: tuple, element_serializers: list[callable]) -> None:
        """
        Serialize a tuple with fixed serializers.
        
        Args:
            values: Tuple of elements to serialize
            element_serializers: List of serializer functions, one per element
        Raises:
            ValueError: If lengths don't match
        """
        if len(values) != len(element_serializers):
            raise ValueError(
                "Tuple length must match the number of serializers.")
        for value, serializer in zip(values, element_serializers):
            serializer(value)

    def serialize_dict(self, dictionary: dict, key_serializer: callable, value_serializer: callable) -> None:
        if len(dictionary) > self.MAX_SEQUENCE_LENGTH:
            raise ValueError(f"Dictionary length {len(dictionary)} exceeds maximum {self.MAX_SEQUENCE_LENGTH}")
        self.serialize_u32(len(dictionary))
        for key, value in dictionary.items():
            key_serializer(key)
            value_serializer(value)

    def serialize_address(self, address: str) -> None:
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
        Serializes an array of uint8 values with ULEB128 length prefix.
        
        Args:
            array (list): List of integers (0-255) representing uint8 values.
        Raises:
            TypeError: If array is not a list
            ValueError: If any value is outside uint8 range or length exceeds max
        """
        if not isinstance(array, list):
            raise TypeError("Expected a list of uint8 values.")
        if any(not (0 <= value < 256) for value in array):
            raise ValueError(
                "All elements in the array must be in the range 0-255.")

        if len(array) > self.MAX_SEQUENCE_LENGTH:
            raise ValueError(f"Sequence length {len(array)} exceeds maximum {self.MAX_SEQUENCE_LENGTH}")

        self.buffer.extend(decimal_to_bcs(len(array)))  # Serialize length as a single byte (u8)
        self.buffer.extend(array)     # Append raw uint8 values

    def _serialize_integer(self, value: int, byte_size: int, format_char: str) -> None:
        """
        Internal method to serialize integers of various sizes.
        
        Args:
            value: Integer to serialize
            byte_size: Number of bytes (1, 2, 4, or 8)
            format_char: struct format character ('B', 'H', 'I', or 'Q')
        Raises:
            ValueError: If value outside range for byte_size
        """
        if not (0 <= value < 2**(byte_size * 8)):
            raise ValueError(
                f"Value out of range for {byte_size * 8}-bit integer.")
        # Little-endian format
        self.buffer.extend(struct.pack("<" + format_char, value))

    def serialize_bytes(self, bytes_data: bytes) -> None:
        """
        Serialize bytes with u32 length prefix.
        
        Args:
            bytes_data: Raw bytes to serialize
        Raises:
            ValueError: If length exceeds MAX_SEQUENCE_LENGTH
        """
        if len(bytes_data) > self.MAX_SEQUENCE_LENGTH:
            raise ValueError(f"Sequence length {len(bytes_data)} exceeds maximum {self.MAX_SEQUENCE_LENGTH}")
        self.serialize_u32(len(bytes_data))
        self.buffer.extend(bytes_data)

    def get_bytes(self) -> bytes:
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


def decimal_to_bcs(num: int) -> list[int]:
    """
    Convert integer to ULEB128 encoding.
    
    Args:
        num: Non-negative integer to encode
    Returns:
        List of bytes in ULEB128 format
    """
    if num == 0:
        return [0]  # Need to handle zero case
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
