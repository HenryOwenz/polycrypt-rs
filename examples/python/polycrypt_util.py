import ctypes
import os
import json
from typing import List, Dict, Any

# Load the shared library
# lib_path = os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'release', 'libpolycrypt_rs.dylib')
lib_path = os.path.join(os.path.dirname(__file__), '..', '..', 'target', 'release', 'libpolycrypt_rs.so')
try:
    lib = ctypes.CDLL(lib_path)
except OSError as e:
    print(f"Error loading the library: {e}")
    raise

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

class PolyCrypt:
    def __init__(self, key: bytes):
        if len(key) != 32:
            raise ValueError("Key must be 32 bytes long")
        self.key = key

    def encrypt(self, plaintext: bytes) -> bytes:
        plaintext_ptr = (ctypes.c_uint8 * len(plaintext)).from_buffer_copy(plaintext)
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
        result = lib.encrypt(plaintext_ptr, len(plaintext), key_ptr)
        if result.data is None:
            raise ValueError("Encryption failed")
        encrypted = bytes(result.data[:result.len])
        lib.free_byte_array(result)
        return encrypted

    def decrypt(self, ciphertext: bytes) -> bytes:
        ciphertext_ptr = (ctypes.c_uint8 * len(ciphertext)).from_buffer_copy(ciphertext)
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
        result = lib.decrypt(ciphertext_ptr, len(ciphertext), key_ptr)
        if result.data is None:
            raise ValueError("Decryption failed")
        decrypted = bytes(result.data[:result.len])
        lib.free_byte_array(result)
        return decrypted

    def encrypt_fields(self, record: Dict[str, Any], fields_to_encrypt: List[str]) -> Dict[str, Any]:
        record_json = json.dumps(record).encode('utf-8')
        fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
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

    def decrypt_fields(self, record: Dict[str, Any], fields_to_decrypt: List[str]) -> Dict[str, Any]:
        record_json = json.dumps(record).encode('utf-8')
        fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
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

    def encrypt_fields_in_batch(self, records: List[Dict[str, Any]], fields_to_encrypt: List[str]) -> List[Dict[str, Any]]:
        records_json = json.dumps(records).encode('utf-8')
        fields_json = json.dumps(fields_to_encrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
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

    def decrypt_fields_in_batch(self, records: List[Dict[str, Any]], fields_to_decrypt: List[str]) -> List[Dict[str, Any]]:
        records_json = json.dumps(records).encode('utf-8')
        fields_json = json.dumps(fields_to_decrypt).encode('utf-8')
        key_ptr = (ctypes.c_uint8 * len(self.key)).from_buffer_copy(self.key)
        
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

def init_logger():
    lib.init_logger()
