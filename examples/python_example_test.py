import unittest
from python_example import encrypt, decrypt, encrypt_fields, decrypt_fields

class TestPolycryptFFI(unittest.TestCase):
    def setUp(self):
        self.key = bytes([0] * 32)

    def test_encrypt_decrypt(self):
        plaintext = b"Hello, world!"
        encrypted = encrypt(plaintext, self.key)
        self.assertNotEqual(plaintext, encrypted)
        decrypted = decrypt(encrypted, self.key)
        self.assertEqual(plaintext, decrypted)

    def test_field_encryption_decryption(self):
        record = {
            "id": "1234",
            "name": "John Doe",
            "sensitive_data": "This is sensitive information",
            "array_field": ["item1", "item2", "item3"]
        }
        fields_to_encrypt = ["sensitive_data", "array_field"]

        encrypted_record = encrypt_fields(record, fields_to_encrypt, self.key)
        self.assertNotEqual(record["sensitive_data"], encrypted_record["sensitive_data"])
        self.assertNotEqual(record["array_field"], encrypted_record["array_field"])

        decrypted_record = decrypt_fields(encrypted_record, fields_to_encrypt, self.key)
        self.assertEqual(record, decrypted_record)

if __name__ == '__main__':
    unittest.main()
