import ctypes
import os
import json
import sys
import traceback

# Load the shared library
lib_path = os.path.join(os.path.dirname(__file__), '..', 'target', 'release', 'libpolycrypt_rs.so')
try:
    lib = ctypes.CDLL(lib_path)
except OSError as e:
    print(f"Error loading the library: {e}")
    sys.exit(1)

# Define the ByteArray struct
class ByteArray(ctypes.Structure):
    _fields_ = [("data", ctypes.POINTER(ctypes.c_uint8)),
                ("len", ctypes.c_size_t)]

# Define the function signatures
lib.encrypt.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt.restype = ByteArray

lib.decrypt.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt.restype = ByteArray

lib.free_byte_array.argtypes = [ByteArray]
lib.free_byte_array.restype = None

lib.init_logger.argtypes = []
lib.init_logger.restype = None

lib.decrypt_fields.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt_fields.restype = ctypes.c_void_p

lib.encrypt_fields.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt_fields.restype = ctypes.c_void_p

lib.decrypt_fields_in_batch.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt_fields_in_batch.restype = ctypes.c_void_p

lib.encrypt_fields_in_batch.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt_fields_in_batch.restype = ctypes.c_void_p

lib.free_c_char.argtypes = [ctypes.c_void_p]
lib.free_c_char.restype = None

def encrypt(plaintext: bytes, key: bytes) -> bytes:
    plaintext_ptr = (ctypes.c_uint8 * len(plaintext)).from_buffer_copy(plaintext)
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.encrypt(plaintext_ptr, len(plaintext), key_ptr)
    if result.data is None:
        raise ValueError("Encryption failed")
    encrypted = bytes(result.data[:result.len])
    lib.free_byte_array(result)
    return encrypted

def decrypt(ciphertext: bytes, key: bytes) -> bytes:
    ciphertext_ptr = (ctypes.c_uint8 * len(ciphertext)).from_buffer_copy(ciphertext)
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.decrypt(ciphertext_ptr, len(ciphertext), key_ptr)
    if result.data is None:
        raise ValueError("Decryption failed")
    decrypted = bytes(result.data[:result.len])
    lib.free_byte_array(result)
    return decrypted

def decrypt_fields(record: dict, fields_to_decrypt: list, key: bytes) -> dict:
    record_json = json.dumps(record).encode('utf-8')
    fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.decrypt_fields(record_json, fields_json, key_ptr)
    if result is None:
        raise ValueError("Field decryption failed")
    try:
        decrypted_str = ctypes.cast(result, ctypes.c_char_p).value.decode('utf-8')
        decrypted = json.loads(decrypted_str)
        return decrypted
    finally:
        if result:
            lib.free_c_char(result)

def encrypt_fields(record: dict, fields_to_encrypt: list, key: bytes) -> dict:
    record_json = json.dumps(record).encode('utf-8')
    fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.encrypt_fields(record_json, fields_json, key_ptr)
    if result is None:
        raise ValueError("Field encryption failed")
    try:
        encrypted_str = ctypes.cast(result, ctypes.c_char_p).value.decode('utf-8')
        encrypted = json.loads(encrypted_str)
        return encrypted
    finally:
        if result:
            lib.free_c_char(result)

def decrypt_fields_in_batch(records: list, fields_to_decrypt: list, key: bytes) -> list:
    records_json = json.dumps(records).encode('utf-8')
    fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.decrypt_fields_in_batch(records_json, fields_json, key_ptr)
    if result is None:
        raise ValueError("Batch field decryption failed")
    try:
        decrypted_str = ctypes.cast(result, ctypes.c_char_p).value.decode('utf-8')
        decrypted = json.loads(decrypted_str)
        return decrypted
    finally:
        if result:
            lib.free_c_char(result)

def encrypt_fields_in_batch(records: list, fields_to_encrypt: list, key: bytes) -> list:
    records_json = json.dumps(records).encode('utf-8')
    fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
    key_ptr = (ctypes.c_uint8 * len(key)).from_buffer_copy(key)
    
    result = lib.encrypt_fields_in_batch(records_json, fields_json, key_ptr)
    if result is None:
        raise ValueError("Batch field encryption failed")
    try:
        encrypted_str = ctypes.cast(result, ctypes.c_char_p).value.decode('utf-8')
        encrypted = json.loads(encrypted_str)
        return encrypted
    finally:
        if result:
            lib.free_c_char(result)

def main():
    os.environ["RUST_LOG"] = "debug"
    lib.init_logger()

    plaintext = b"Hello, world!"
    key = bytes([0] * 32)  # In a real scenario, use a proper key generation method

    try:
        encrypted = encrypt(plaintext, key)
        print(f"Plaintext: {plaintext}")
        print(f"Encrypted: {encrypted}")
        print(f"Encrypted length: {len(encrypted)}")

        decrypted = decrypt(encrypted, key)
        print(f"Decrypted: {decrypted}")
        print(f"Decrypted length: {len(decrypted)}")

        record = {
            "id": "1234",
            "name": "John Doe",
            "sensitive_data": "This is sensitive information",
            "array_field": ["item1", "item2", "item3"]
        }
        fields_to_encrypt = ["sensitive_data", "array_field"]

        print(f"Original record: {record}")

        encrypted_record = encrypt_fields(record, fields_to_encrypt, key)
        print(f"Encrypted record: {encrypted_record}")

        decrypted_record = decrypt_fields(encrypted_record, fields_to_encrypt, key)
        print(f"Decrypted record: {decrypted_record}")

        # Test batch encryption and decryption
        records = [record, {
            "id": "5678",
            "name": "Jane Smith",
            "sensitive_data": "Another piece of sensitive information",
            "array_field": ["item4", "item5", "item6"]
        }]

        encrypted_records = encrypt_fields_in_batch(records, fields_to_encrypt, key)
        print(f"Encrypted records: {encrypted_records}")

        decrypted_records = decrypt_fields_in_batch(encrypted_records, fields_to_encrypt, key)
        print(f"Decrypted records: {decrypted_records}")

    except Exception as e:
        print(f"An error occurred: {e}")
        traceback.print_exc()

    print("Program completed successfully")

if __name__ == "__main__":
    main()
