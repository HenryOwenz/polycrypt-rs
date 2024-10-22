package main

import (
	"fmt"
	"os"

	"go-ffi-test/polycrypt"
)

func main() {
	// Set the RUST_LOG environment variable
	os.Setenv("RUST_LOG", "info")

	// Initialize the logger
	polycrypt.InitLogger()

	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)
	// In a real scenario, use a proper key generation method

	pc := polycrypt.NewPolyCrypt(key)

	encrypted, err := pc.Encrypt(plaintext)
	if err != nil {
		fmt.Printf("Encryption error: %v\n", err)
		return
	}

	fmt.Printf("Plaintext: %s\n", plaintext)
	fmt.Printf("Encrypted: %v\n", encrypted)
	fmt.Printf("Encrypted length: %d\n", len(encrypted))

	decrypted, err := pc.Decrypt(encrypted)
	if err != nil {
		fmt.Printf("Decryption error: %v\n", err)
		return
	}

	fmt.Printf("Decrypted: %s\n", decrypted)
	fmt.Printf("Decrypted length: %d\n", len(decrypted))

	// Test encrypt_fields and decrypt_fields
	record := map[string]interface{}{
		"id":             "1234",
		"name":           "John Doe",
		"sensitive_data": "This is sensitive information",
		"array_field":    []string{"item1", "item2", "item3"},
	}
	record2 := map[string]interface{}{
		"id":             "5678",
		"name":           "Jane Smith",
		"sensitive_data": "Another piece of sensitive information",
		"array_field":    []string{"item4", "item5", "item6"},
	}
	records := []map[string]interface{}{record, record2}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}

	fmt.Printf("Original record: %v\n", record)

	encryptedRecord, err := pc.EncryptFields(record, fieldsToEncrypt)
	if err != nil {
		fmt.Printf("Field encryption error: %v\n", err)
		return
	}
	fmt.Printf("Encrypted record: %v\n", encryptedRecord)

	decryptedRecord, err := pc.DecryptFields(encryptedRecord, fieldsToEncrypt)
	if err != nil {
		fmt.Printf("Field decryption error: %v\n", err)
		return
	}
	fmt.Printf("Decrypted record: %v\n", decryptedRecord)

	// Test encrypt_fields_in_batch and decrypt_fields_in_batch
	encryptedRecords, err := pc.EncryptFieldsInBatch(records, fieldsToEncrypt)
	if err != nil {
		fmt.Printf("Batch field encryption error: %v\n", err)
		return
	}
	fmt.Printf("Encrypted batch records: %v\n", encryptedRecords)

	decryptedRecords, err := pc.DecryptFieldsInBatch(encryptedRecords, fieldsToEncrypt)
	if err != nil {
		fmt.Printf("Batch field decryption error: %v\n", err)
		return
	}
	fmt.Printf("Decrypted batch records: %v\n", decryptedRecords)
}
