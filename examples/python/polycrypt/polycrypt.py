import ctypes
import os
import json
import sys
import platform

# Determine the appropriate library extension based on the platform
def get_lib_name():
    system = platform.system().lower()
    if system == 'linux':
        return 'libpolycrypt_rs.so'
    elif system == 'darwin':  # macOS
        return 'libpolycrypt_rs.dylib'
    elif system == 'windows':
        return 'libpolycrypt_rs.dll'
    else:
        raise OSError(f"Unsupported platform: {system}")

# Load the shared library
lib_name = get_lib_name()
lib_path = os.path.join(os.path.dirname(__file__), lib_name)
try:
    lib = ctypes.CDLL(lib_path)
except OSError as e:
    print(f"Error loading the library: {e}")
    print(f"Attempted to load: {lib_path}")
    sys.exit(1)

# Define the ByteArray struct
class ByteArray(ctypes.Structure):
    _fields_ = [("data", ctypes.POINTER(ctypes.c_uint8)),
                ("len", ctypes.c_size_t)]

# Define the FFIResult struct
class FFIResult(ctypes.Structure):
    _fields_ = [("data", ByteArray),
                ("error_code", ctypes.c_int32)]

# Define function signatures
lib.encrypt.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt.restype = FFIResult
lib.decrypt.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt.restype = FFIResult
lib.encrypt_fields.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt_fields.restype = FFIResult
lib.decrypt_fields.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt_fields.restype = FFIResult
lib.encrypt_fields_in_batch.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.encrypt_fields_in_batch.restype = FFIResult
lib.decrypt_fields_in_batch.argtypes = [ctypes.POINTER(ctypes.c_uint8), ctypes.c_size_t, ctypes.c_char_p, ctypes.POINTER(ctypes.c_uint8)]
lib.decrypt_fields_in_batch.restype = FFIResult
lib.free_ffi_result.argtypes = [FFIResult]
lib.free_ffi_result.restype = None
lib.init_logger.argtypes = []
lib.init_logger.restype = None

class PolyCrypt:
    def __init__(self, key):
        if len(key) != 32:
            raise ValueError("Key must be 32 bytes long")
        self.key = key

    def encrypt(self, plaintext):
        plaintext_ptr = (ctypes.c_uint8 * len(plaintext)).from_buffer_copy(plaintext)
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        result = lib.encrypt(plaintext_ptr, len(plaintext), key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Encryption failed")
        encrypted = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return encrypted

    def decrypt(self, ciphertext):
        ciphertext_ptr = (ctypes.c_uint8 * len(ciphertext)).from_buffer_copy(ciphertext)
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        result = lib.decrypt(ciphertext_ptr, len(ciphertext), key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Decryption failed")
        decrypted = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return decrypted

    def encrypt_fields(self, record, fields_to_encrypt):
        record_json = json.dumps(record).encode('utf-8')
        fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        result = lib.encrypt_fields(record_json, fields_json, key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Field encryption failed")
        encrypted_json = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return json.loads(encrypted_json)

    def decrypt_fields(self, encrypted_record, fields_to_decrypt):
        encrypted_json = json.dumps(encrypted_record).encode('utf-8')
        fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        encrypted_ptr = (ctypes.c_uint8 * len(encrypted_json)).from_buffer_copy(encrypted_json)
        result = lib.decrypt_fields(encrypted_ptr, len(encrypted_json), fields_json, key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Field decryption failed")
        decrypted_json = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return json.loads(decrypted_json)

    def encrypt_fields_in_batch(self, records, fields_to_encrypt):
        records_json = json.dumps(records).encode('utf-8')
        fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        result = lib.encrypt_fields_in_batch(records_json, fields_json, key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Batch field encryption failed")
        encrypted_json = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return json.loads(encrypted_json)

    def decrypt_fields_in_batch(self, encrypted_records, fields_to_decrypt):
        encrypted_json = json.dumps(encrypted_records).encode('utf-8')
        fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        encrypted_ptr = (ctypes.c_uint8 * len(encrypted_json)).from_buffer_copy(encrypted_json)
        result = lib.decrypt_fields_in_batch(encrypted_ptr, len(encrypted_json), fields_json, key_ptr)
        if result.error_code != 0:
            lib.free_ffi_result(result)
            raise ValueError("Batch field decryption failed")
        decrypted_json = bytes(result.data.data[:result.data.len])
        lib.free_ffi_result(result)
        return json.loads(decrypted_json)

def init_logger():
    lib.init_logger()
