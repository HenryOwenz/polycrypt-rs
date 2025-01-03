import unittest
from polycrypt.polycrypt import PolyCrypt, init_logger

class TestPolyCrypt(unittest.TestCase):
    def setUp(self):
        self.key = b'0' * 32
        self.pc = PolyCrypt(self.key)
        init_logger()

    def test_encrypt_decrypt(self):
        plaintext = b"Hello, world!"
        encrypted = self.pc.encrypt(plaintext)
        decrypted = self.pc.decrypt(encrypted)
        self.assertEqual(plaintext, decrypted)

    def test_encrypt_decrypt_fields(self):
        record = {
            "id": "1234",
            "name": "John Doe",
            "sensitive_data": "This is sensitive information",
            "array_field": ["item1", "item2", "item3"]
        }
        fields_to_encrypt = ["sensitive_data", "array_field"]

        encrypted_record = self.pc.encrypt_fields(record, fields_to_encrypt)
        self.assertNotEqual(encrypted_record["sensitive_data"], record["sensitive_data"])
        self.assertNotEqual(encrypted_record["array_field"], record["array_field"])

        decrypted_record = self.pc.decrypt_fields(encrypted_record, fields_to_encrypt)
        self.assertEqual(record, decrypted_record)

    def test_encrypt_decrypt_fields_in_batch(self):
        records = [
            {
                "id": "1234",
                "name": "John Doe",
                "sensitive_data": "This is sensitive information",
                "array_field": ["item1", "item2", "item3"]
            },
            {
                "id": "5678",
                "name": "Jane Smith",
                "sensitive_data": "Another piece of sensitive information",
                "array_field": ["item4", "item5", "item6"]
            }
        ]
        fields_to_encrypt = ["sensitive_data", "array_field"]

        encrypted_records = self.pc.encrypt_fields_in_batch(records, fields_to_encrypt)
        for record, encrypted_record in zip(records, encrypted_records):
            self.assertNotEqual(encrypted_record["sensitive_data"], record["sensitive_data"])
            self.assertNotEqual(encrypted_record["array_field"], record["array_field"])

        decrypted_records = self.pc.decrypt_fields_in_batch(encrypted_records, fields_to_encrypt)
        self.assertEqual(records, decrypted_records)

if __name__ == '__main__':
    unittest.main()
