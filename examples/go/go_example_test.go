package main

import (
	"fmt"
	"reflect"
	"testing"
)

// Keep the existing main() function

func TestEncryptDecrypt(t *testing.T) {
	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)

	encrypted, err := encrypt(plaintext, key)
	if err != nil {
		t.Fatalf("Encryption failed: %v", err)
	}
	if len(encrypted) == 0 {
		t.Fatal("Encryption failed: empty result")
	}

	decrypted, err := decrypt(encrypted, key)
	if err != nil {
		t.Fatalf("Decryption failed: %v", err)
	}

	if !reflect.DeepEqual(plaintext, decrypted) {
		t.Errorf("Decrypted text does not match original plaintext")
	}
}

func TestFieldEncryptionDecryption(t *testing.T) {
	record := map[string]interface{}{
		"id":             "1234",
		"name":           "John Doe",
		"sensitive_data": "This is sensitive information",
		"array_field":    []string{"item1", "item2", "item3"},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}
	key := make([]byte, 32)

	encryptedRecord, err := encryptFields(record, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Field encryption failed: %v", err)
	}

	if reflect.DeepEqual(encryptedRecord["sensitive_data"], record["sensitive_data"]) {
		t.Errorf("Sensitive data was not encrypted")
	}

	decryptedRecord, err := decryptFields(encryptedRecord, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Field decryption failed: %v", err)
	}

	if !reflect.DeepEqual(record, decryptedRecord) {
		t.Errorf("Decrypted record does not match original record")
	}
}

func TestBatchFieldEncryptionDecryption(t *testing.T) {
	records := []map[string]interface{}{
		{
			"id":             "1234",
			"name":           "John Doe",
			"sensitive_data": "This is sensitive information",
			"array_field":    []string{"item1", "item2", "item3"},
		},
		{
			"id":             "5678",
			"name":           "Jane Smith",
			"sensitive_data": "Another sensitive information",
			"array_field":    []string{"item4", "item5", "item6"},
		},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}
	key := make([]byte, 32)

	encryptedRecords, err := encryptFieldsInBatch(records, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Batch field encryption failed: %v", err)
	}

	for i, encryptedRecord := range encryptedRecords {
		if reflect.DeepEqual(encryptedRecord["sensitive_data"], records[i]["sensitive_data"]) {
			t.Errorf("Sensitive data was not encrypted in record %d", i)
		}
	}

	decryptedRecords, err := decryptFieldsInBatch(encryptedRecords, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Batch field decryption failed: %v", err)
	}

	if !reflect.DeepEqual(records, decryptedRecords) {
		t.Errorf("Decrypted records do not match original records")
		t.Logf("Original records: %s", printRecords(records))
		t.Logf("Decrypted records: %s", printRecords(decryptedRecords))
		
		for i := range records {
			if !reflect.DeepEqual(records[i], decryptedRecords[i]) {
				t.Errorf("Mismatch in record %d", i)
				t.Logf("Original: %s", printMap(records[i]))
				t.Logf("Decrypted: %s", printMap(decryptedRecords[i]))
				
				for key := range records[i] {
					if !reflect.DeepEqual(records[i][key], decryptedRecords[i][key]) {
						t.Errorf("Mismatch in field %q of record %d", key, i)
						t.Logf("Original: %#v", records[i][key])
						t.Logf("Decrypted: %#v", decryptedRecords[i][key])
					}
				}
			}
		}
	}
}

// Helper function to print a map
func printMap(m map[string]interface{}) string {
	result := "{\n"
	for k, v := range m {
		result += fmt.Sprintf("  %s: %#v\n", k, v)
	}
	result += "}"
	return result
}

// Helper function to print a slice of records
func printRecords(records []map[string]interface{}) string {
	result := "[\n"
	for i, record := range records {
		result += fmt.Sprintf("  Record %d: %s\n", i, printMap(record))
	}
	result += "]"
	return result
}
