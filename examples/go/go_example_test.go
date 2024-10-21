package main

import (
	"fmt"
	"reflect"
	"testing"
)

func TestEncryptDecrypt(t *testing.T) {
	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)
	pc := NewPolyCrypt(key)

	encrypted, err := pc.Encrypt(plaintext)
	if err != nil {
		t.Fatalf("Encryption failed: %v", err)
	}
	if len(encrypted) == 0 {
		t.Fatal("Encryption failed: empty result")
	}

	decrypted, err := pc.Decrypt(encrypted)
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
	pc := NewPolyCrypt(key)

	encryptedRecord, err := pc.EncryptFields(record, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Field encryption failed: %v", err)
	}

	if reflect.DeepEqual(record["sensitive_data"], encryptedRecord["sensitive_data"]) {
		t.Error("Sensitive data was not encrypted")
	}

	if reflect.DeepEqual(record["array_field"], encryptedRecord["array_field"]) {
		t.Error("Array field was not encrypted")
	}

	decryptedRecord, err := pc.DecryptFields(encryptedRecord, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Field decryption failed: %v", err)
	}

	for key, value := range record {
		if !reflect.DeepEqual(value, decryptedRecord[key]) {
			t.Errorf("Mismatch for key %s: original %v (%T), decrypted %v (%T)", key, value, value, decryptedRecord[key], decryptedRecord[key])
		}
	}

	if !reflect.DeepEqual(record, decryptedRecord) {
		t.Errorf("Decrypted record does not match original record.\nOriginal: %+v\nDecrypted: %+v", record, decryptedRecord)
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
			"sensitive_data": "Another piece of sensitive information",
			"array_field":    []string{"item4", "item5", "item6"},
		},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}
	key := make([]byte, 32)
	pc := NewPolyCrypt(key)

	encryptedRecords, err := pc.EncryptFieldsInBatch(records, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Batch field encryption failed: %v", err)
	}

	for i, encryptedRecord := range encryptedRecords {
		if reflect.DeepEqual(encryptedRecord["sensitive_data"], records[i]["sensitive_data"]) {
			t.Errorf("Sensitive data was not encrypted in record %d", i)
		}
	}

	decryptedRecords, err := pc.DecryptFieldsInBatch(encryptedRecords, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Batch field decryption failed: %v", err)
	}

	for i, record := range records {
		for key, value := range record {
			if !reflect.DeepEqual(value, decryptedRecords[i][key]) {
				t.Errorf("Mismatch for key %s in record %d: original %v (%T), decrypted %v (%T)", 
					key, i, value, value, decryptedRecords[i][key], decryptedRecords[i][key])
			}
		}
	}

	if !reflect.DeepEqual(records, decryptedRecords) {
		t.Errorf("Decrypted records do not match original records.\nOriginal: %s\nDecrypted: %s", printRecords(records), printRecords(decryptedRecords))
	}
}

// Add these helper functions at the end of the file
func printMap(m map[string]interface{}) string {
	result := "{\n"
	for k, v := range m {
		result += fmt.Sprintf("  %s: %#v\n", k, v)
	}
	result += "}"
	return result
}

func printRecords(records []map[string]interface{}) string {
	result := "[\n"
	for i, record := range records {
		result += fmt.Sprintf("  Record %d: %s\n", i, printMap(record))
	}
	result += "]"
	return result
}
