import os
from polycrypt.polycrypt import PolyCrypt, init_logger

def main():
    os.environ["RUST_LOG"] = "debug"
    init_logger()

    plaintext = b"Hello, world!"
    key = bytes([0] * 32)  # In a real scenario, use a proper key generation method

    pc = PolyCrypt(key)

    try:
        encrypted = pc.encrypt(plaintext)
        print(f"Plaintext: {plaintext}")
        print(f"Encrypted: {encrypted}")
        print(f"Encrypted length: {len(encrypted)}")

        decrypted = pc.decrypt(encrypted)
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

        encrypted_record = pc.encrypt_fields(record, fields_to_encrypt)
        print(f"Encrypted record: {encrypted_record}")

        decrypted_record = pc.decrypt_fields(encrypted_record, fields_to_encrypt)
        print(f"Decrypted record: {decrypted_record}")

        # Test batch encryption and decryption
        records = [record, {
            "id": "5678",
            "name": "Jane Smith",
            "sensitive_data": "Another piece of sensitive information",
            "array_field": ["item4", "item5", "item6"]
        }]

        encrypted_records = pc.encrypt_fields_in_batch(records, fields_to_encrypt)
        print(f"Encrypted records: {encrypted_records}")

        decrypted_records = pc.decrypt_fields_in_batch(encrypted_records, fields_to_encrypt)
        print(f"Decrypted records: {decrypted_records}")

    except Exception as e:
        print(f"An error occurred: {e}")
        import traceback
        traceback.print_exc()

    print("Program completed successfully")

if __name__ == "__main__":
    main()
